use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use rand::RngExt;

use crate::{
    components::*,
    entities::{
        Human,
        ball::*,
        paddle::{PADDLE_SPEED, Paddle},
    },
    resources::{AddAnotherPatientEvent, FellThrough, Score},
};

// System: project positions to transforms
pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

pub fn move_ball(balls: Query<(&mut Position, &Velocity), With<Ball>>) {
    for (mut position, velocity) in balls {
        position.0 += velocity.0 * FALL_SPEED;
    }
}

pub fn apply_gravity(velocities: Query<&mut Velocity, With<Ball>>) {
    for mut velocity in velocities {
        if velocity.0.y > -FALL_SPEED {
            velocity.0.y -= 0.1;
        }
    }
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
pub fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

const BOUNCE_UP_SPEED: f32 = 6.0;

pub fn handle_collisions(
    balls: Query<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    for (mut ball_velocity, ball_position, ball_collider) in balls {
        for (other_position, other_collider) in &other_things {
            if let Some(collision) = collide_with_side(
                Aabb2d::new(ball_position.0, ball_collider.half_size()),
                Aabb2d::new(other_position.0, other_collider.half_size()),
            ) {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top => {
                        println!("Bounce it up!");
                        let random_number = rng.random_range((-1. * FALL_SPEED)..FALL_SPEED);
                        ball_velocity.0.y = FALL_SPEED * BOUNCE_UP_SPEED;
                        ball_velocity.0.x += random_number;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}

pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Human>>,
    mut commands: Commands,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        paddle_velocity.0.x = -PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        paddle_velocity.0.x = PADDLE_SPEED;
    } else {
        paddle_velocity.0.x = 0.;
    }
    if keyboard_input.just_released(KeyCode::Space) {
        commands.trigger(AddAnotherPatientEvent);
    }
}

pub fn move_paddles(mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>) {
    for (mut position, velocity) in &mut paddles {
        position.0 += velocity.0;
    }
}

pub fn constrain_paddle_position(
    mut paddles: Query<(&mut Position, &Collider), (With<Paddle>, Without<Gutter>)>,
    gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>)>,
) {
    for (mut paddle_position, paddle_collider) in &mut paddles {
        for (gutter_position, gutter_collider) in &gutters {
            let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
            let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

            if let Some(collision) = collide_with_side(paddle_aabb, gutter_aabb) {
                match collision {
                    Collision::Top => {
                        paddle_position.0.y = gutter_position.0.y
                            + gutter_collider.half_size().y
                            + paddle_collider.half_size().y;
                    }
                    Collision::Bottom => {
                        paddle_position.0.y = gutter_position.0.y
                            - gutter_collider.half_size().y
                            - paddle_collider.half_size().y;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn update_score(_event: On<FellThrough>, mut score: ResMut<Score>) {
    score.fell_through += 1;
}

pub fn update_scoreboard(
    mut fell_through_count: Single<&mut Text, With<FellThroughScore>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        fell_through_count.0 = score.fell_through.to_string();
    }
}

pub fn reset_ball(
    event: On<FellThrough>,
    mut balls: Query<(&mut Position, &mut Velocity), With<Ball>>,
    window: Single<&Window>,
) {
    if let Ok(ball) = balls.get_mut(event.ball) {
        println!("Ball must reset!");

        let (mut ball_position, mut ball_velocity) = ball;
        let half_window_size = window.resolution.size() / 2.;
        ball_position.0 = Vec2::new(0., half_window_size.y);
        ball_velocity.0 = Vec2::ZERO;
    }
}

pub fn add_another_patient(
    _event: On<AddAnotherPatientEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    println!("Should add another one!");
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);
    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

pub fn detect_fell_through(
    balls: Query<(Entity, (&Position, &Collider)), With<Ball>>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    for (entity, (ball_position, ball_collider)) in &balls {
        let half_window_size = window.resolution.size() / 2.;

        if ball_position.0.y - ball_collider.half_size().y < -half_window_size.y {
            commands.trigger(FellThrough { ball: entity });
        }
    }
}

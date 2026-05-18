use std::ops::Deref;

use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use rand::RngExt;

use crate::{
    components::*,
    resources::{BouncedEvent, FellThrough, PlayerBouncedEvent, Score, WaveTimerResource},
    settings::*,
};

// System: project positions to transforms
pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

pub fn move_ball(patients: Query<(&mut Position, &Velocity), With<Patient>>) {
    for (mut position, velocity) in patients {
        position.0 += velocity.0 * FALL_SPEED;
    }
}

pub fn apply_gravity(velocities: Query<&mut Velocity, With<Patient>>) {
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

pub fn handle_collisions(
    balls: Query<(Entity, &mut Velocity, &Position, &Collider), With<Patient>>,
    other_things: Query<(&Position, &Collider), Without<Patient>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mut commands: Commands,
) {
    for (ball_entity, mut ball_velocity, ball_position, ball_collider) in balls {
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
                        if ball_velocity.0.y.signum() == -1.0 {
                            println!("Switch direction from down to up");
                            ball_velocity.0.y *= -1.;
                        }

                        // println!("Bounce it up!");
                        commands.trigger(BouncedEvent {
                            patient: ball_entity,
                        });
                        let random_number = rng.random_range((-1. * FALL_SPEED)..FALL_SPEED);
                        // ball_velocity.0.y = FALL_SPEED * BOUNCE_UP_SPEED;
                        ball_velocity.0.x += random_number;
                    }
                    Collision::Bottom => {
                        // ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}

// pub fn handle_player_bump_ball(
//     balls: Query<(&mut Velocity, &Position, &Collider), With<Patient>>,
//     player: Single<(&Position, &Collider), With<Human>>,
//     mut commands: Commands,
// ) {
//     for (mut ball_velocity, ball_position, ball_collider) in balls {
//         let (other_position, other_collider) = player.deref();
//         // let other_position = player.0;
//         // let other_collider = player.1;
//         if let Some(collision) = collide_with_side(
//             Aabb2d::new(ball_position.0, ball_collider.half_size()),
//             Aabb2d::new(other_position.0, other_collider.half_size()),
//         ) {
//             if matches!(collision, Collision::Top) {
//                 if ball_velocity.0.y.signum() == 1.0 {
//                     println!("Bounce it up!");
//                     ball_velocity.0.y *= FALL_SPEED * BOUNCE_UP_SPEED;
//                 }
//                 commands.trigger(PlayerBouncedEvent);
//             }
//         }
//     }
// }

pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Human>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        paddle_velocity.0.x = -PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        paddle_velocity.0.x = PADDLE_SPEED;
    } else {
        paddle_velocity.0.x = 0.;
    }
}

pub fn move_paddles(mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>) {
    for (mut position, velocity) in &mut paddles {
        position.0 += velocity.0;
    }
}

pub fn move_agents(
    agents: Query<(&Position, &mut Velocity), (With<Paddle>, With<Agent>)>,
    window: Single<&Window>,
) {
    for (position, mut velocity) in agents {
        if position.0.x < -(window.resolution.width() / 3.0) {
            velocity.0.x = AGENT_SPEED;
        }
        if position.0.x > window.resolution.width() / 3.0 {
            velocity.0.x = -AGENT_SPEED;
        }
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
                    Collision::Right => {
                        paddle_position.0.x = gutter_position.0.x
                            + gutter_collider.half_size().x
                            + paddle_collider.half_size().x;
                    }
                    Collision::Left => {
                        paddle_position.0.x = gutter_position.0.x
                            - gutter_collider.half_size().x
                            - paddle_collider.half_size().x;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn update_scoreboard(
    mut fell_through_count: Single<&mut Text, With<FellThroughScore>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        fell_through_count.0 = score.fell_through.to_string();
    }
}

pub fn detect_fell_through(
    balls: Query<(Entity, (&Position, &Collider)), With<Patient>>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    for (entity, (ball_position, ball_collider)) in &balls {
        let half_window_size = window.resolution.size() / 2.;

        if ball_position.0.y - ball_collider.half_size().y < -half_window_size.y {
            commands.trigger(FellThrough { patient: entity });
        }
    }
}

pub fn tick_wave_timer(
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimerResource>,
    commands: Commands,
    rng: Single<&mut WyRand, With<GlobalRng>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    wave_timer.timer.tick(time.delta());

    if wave_timer.timer.just_finished() {
        println!("Timer finished; spawn new patient!");
        spawn_patient(commands, rng, asset_server, window);
    }
}

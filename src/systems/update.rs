use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use rand::RngExt;

use crate::{
    components::*,
    entities::{Human, ball::*, paddle::Paddle},
    resources::FellThrough,
};

// System: project positions to transforms
pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

pub fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0 * BALL_SPEED;
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
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    let random_number = rng.random_range((-1. * BALL_SPEED)..BALL_SPEED);

    for (other_position, other_collider) in &other_things {
        if let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) {
            println!("Collision {:?} + {:.4}", collision, random_number);
            match collision {
                Collision::Left => {
                    ball_velocity.0.x *= -1.;
                    ball_velocity.0.y += random_number;
                }
                Collision::Right => {
                    ball_velocity.0.x *= -1.;
                    ball_velocity.0.y += random_number;
                }
                Collision::Top => {
                    ball_velocity.0.y *= -1.;
                }
                Collision::Bottom => {
                    ball_velocity.0.y *= -1.;
                }
            }
        }
    }
}

const PADDLE_SPEED: f32 = 5.;

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

pub fn reset_ball(
    _event: On<FellThrough>,
    ball: Single<(&mut Position, &mut Velocity), With<Ball>>,
) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    ball_position.0 = Vec2::ZERO;
    ball_velocity.0 = Vec2::new(0., -BALL_SPEED);
}

pub fn detect_fell_through(
    ball: Single<(Entity, (&Position, &Collider)), With<Ball>>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    let (entity, (ball_position, ball_collider)) = ball.into_inner();
    let half_window_size = window.resolution.size() / 2.;

    // if ball_position.0.y + ball_collider.half_size().y > half_window_size.y {
    //     commands.trigger(FellThrough { ball: entity });
    // }
    if ball_position.0.y - ball_collider.half_size().y < -half_window_size.y {
        commands.trigger(FellThrough { ball: entity });
    }
}

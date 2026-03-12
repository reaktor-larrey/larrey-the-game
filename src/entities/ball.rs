use bevy::prelude::*;

use crate::components::{Collider, Position, Velocity};

const BALL_SIZE: f32 = 10.0;
pub const BALL_SPEED: f32 = 3.;

pub const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
pub const BALL_COLOR: Color = Color::srgb(1., 0., 0.);

#[derive(Component)]
#[require(
    Position,
    Velocity = Velocity(Vec2::new(BALL_SPEED, 0.0)),
    Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
)]
pub struct Ball;

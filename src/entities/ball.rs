use bevy::prelude::*;

use crate::components::{Collider, Position, Velocity};

const BALL_SIZE: f32 = 10.0;
pub const FALL_SPEED: f32 = 1.5;

pub const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
pub const BALL_COLOR: Color = Color::srgb(1., 0., 0.);

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
)]
pub struct Ball;

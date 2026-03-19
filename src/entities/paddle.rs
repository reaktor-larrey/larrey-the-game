use bevy::prelude::*;

use crate::components::{Collider, Position, Velocity};

pub const PADDLE_WIDTH: f32 = 100.;
pub const PADDLE_HEIGHT: f32 = 10.0;
pub const PADDLE_SHAPE: Rectangle = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

pub const PADDLE_SPEED: f32 = 5.;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(PADDLE_SHAPE),
    Sprite
)]
pub struct Paddle;

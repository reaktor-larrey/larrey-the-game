use bevy::prelude::*;

use crate::components::{Collider, Position, Velocity};

pub const PADDLE_SHAPE: Rectangle = Rectangle::new(60., 20.);
pub const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

pub const PADDLE_SPEED: f32 = 5.;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(PADDLE_SHAPE)
)]
pub struct Paddle;

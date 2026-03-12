use bevy::prelude::*;

use crate::components::{Collider, Position};

pub const PADDLE_SHAPE: Rectangle = Rectangle::new(20., 50.);
pub const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

#[derive(Component)]
#[require(
    Position,
    Collider = Collider(PADDLE_SHAPE)
)]
pub struct Paddle;

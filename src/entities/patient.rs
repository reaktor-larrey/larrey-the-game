use bevy::prelude::*;

use crate::components::{Collider, Position, Velocity};

pub const BALL_SIZE: f32 = 64.0;
pub const FALL_SPEED: f32 = 1.5;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
    Sprite

)]
pub struct Patient;

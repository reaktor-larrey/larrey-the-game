use bevy::prelude::*;

pub mod agent;
pub mod patient;
pub mod player;

pub use agent::*;
pub use patient::*;
pub use player::*;

#[derive(Component)]
pub struct Human;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component, Default)]
pub struct Collider(pub Rectangle);

#[derive(Component)]
#[require(Position, Collider)]
pub struct Gutter;

pub const GUTTER_COLOR: Color = Color::srgb_u8(20, 133, 101);
pub const GUTTER_THICKNESS: f32 = 20.;

#[derive(Component)]
pub struct FellThroughScore;

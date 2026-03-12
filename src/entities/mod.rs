use bevy::ecs::component::Component;

pub mod ball;
pub mod paddle;

#[derive(Component)]
pub struct Human;

#[derive(Component)]
pub struct Computer;

use bevy::ecs::{entity::Entity, event::EntityEvent, resource::Resource};

#[derive(Resource)]
pub struct Score {
    pub human: u32,
    pub computer: u32,
}

#[derive(EntityEvent)]
pub struct Scored {
    #[event_target]
    pub scorer: Entity,
}

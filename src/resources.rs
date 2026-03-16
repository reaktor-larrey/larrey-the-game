use bevy::ecs::{entity::Entity, event::EntityEvent};

#[derive(EntityEvent)]
pub struct FellThrough {
    #[event_target]
    pub ball: Entity,
}

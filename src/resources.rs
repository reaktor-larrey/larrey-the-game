use bevy::ecs::{
    entity::Entity,
    event::{EntityEvent, Event},
    resource::Resource,
};

#[derive(Resource)]
pub struct Score {
    pub fell_through: u32,
}

#[derive(EntityEvent)]
pub struct FellThrough {
    #[event_target]
    pub ball: Entity,
}

#[derive(Event)]
pub struct AddAnotherPatientEvent;

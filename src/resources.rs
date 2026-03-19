use bevy::{
    ecs::{
        entity::Entity,
        event::{EntityEvent, Event},
        resource::Resource,
    },
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct Score {
    pub fell_through: i32,
}

#[derive(Resource)]
pub struct WaveTimerResource {
    pub timer: Timer,
}

impl WaveTimerResource {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

impl Default for WaveTimerResource {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(EntityEvent)]
pub struct FellThrough {
    #[event_target]
    pub patient: Entity,
}

#[derive(Event)]
pub struct AddAnotherPatientEvent;

#[derive(Event)]
pub struct BouncedEvent;

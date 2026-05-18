use bevy::{
    asset::{AssetServer, Handle},
    audio::AudioSource,
    ecs::{
        entity::Entity,
        event::{EntityEvent, Event},
        resource::Resource,
        world::{FromWorld, World},
    },
    prelude::Deref,
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

#[derive(Resource, Deref)]
pub struct SoundEffect {
    handle: Handle<AudioSource>,
}

impl FromWorld for SoundEffect {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        SoundEffect {
            handle: asset_server.load("bounce.mp3"),
        }
    }
}

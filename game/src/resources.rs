use bevy::{
    asset::Handle,
    audio::AudioSource,
    ecs::{
        component::Component,
        entity::Entity,
        event::{EntityEvent, Event},
        resource::Resource,
    },
    state::state::States,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct GameSession {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct Score {
    pub helped: isize,
    pub fell_through: isize,
    pub capacity: isize,
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

#[derive(EntityEvent)]
pub struct BouncedEvent {
    #[event_target]
    pub patient: Entity,
    pub bouncer: Entity,
}

#[derive(Event)]
pub struct PlayerBounceEvent;

#[derive(Resource, Component)]
pub struct SoundEffect {
    pub player_sound: Handle<AudioSource>,
    pub agent_sound: Handle<AudioSource>,
    pub fall_sound: Handle<AudioSource>,
    pub gameover_sound: Handle<AudioSource>,
    pub robot_sound: Handle<AudioSource>,
    pub whee_sound: Handle<AudioSource>,
}

#[derive(Event)]
pub struct AddAnotherPatientEvent;

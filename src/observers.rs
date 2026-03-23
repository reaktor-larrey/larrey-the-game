use bevy::prelude::*;
use bevy_rand::global::GlobalRng;
use bevy_rand::prelude::WyRand;
use rand::RngExt;

use crate::components::*;
use crate::resources::*;
use crate::settings::FALL_SPEED;

pub fn reset_patient(
    event: On<FellThrough>,
    mut patients: Query<(&mut Position, &mut Velocity), With<Patient>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    window: Single<&Window>,
) {
    if let Ok(patient) = patients.get_mut(event.patient) {
        let (mut position, mut velocity) = patient;
        let half_window_size = window.resolution.size() / 2.;
        let random_position = rng.random_range(-half_window_size.x..half_window_size.x);
        position.0 = Vec2::new(random_position, half_window_size.y);
        let random_speed = rng.random_range((-1. * FALL_SPEED)..FALL_SPEED);
        velocity.0 = Vec2::new(random_speed, 0.);
    }
}

pub fn minus_one(_event: On<FellThrough>, mut score: ResMut<Score>) {
    if score.fell_through > 0 {
        score.fell_through -= 1;
    }
}

pub fn plus_one(_event: On<BouncedEvent>, mut score: ResMut<Score>) {
    score.fell_through += 1;
}

pub fn add_another_patient(
    _event: On<AddAnotherPatientEvent>,
    commands: Commands,
    rng: Single<&mut WyRand, With<GlobalRng>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    println!("Should add another one!");
    spawn_patient(commands, rng, asset_server, window);
}

pub fn possibly_add_agent(
    _event: On<BouncedEvent>,
    agents: Query<&Agent>,
    commands: Commands,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    if score.fell_through > 0 {
        if (score.fell_through as u32).is_power_of_two() && score.fell_through > 1 {
            println!("Score: {}: Should add an agent?", score.fell_through);
            let log_score = score.fell_through.ilog2();
            println!("{} agents count vs {} log_score", agents.count(), log_score);
            if agents.count() as u32 <= log_score {
                spawn_agent(commands, asset_server, window);
            }
        }
    }
}

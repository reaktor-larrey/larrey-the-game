use bevy::prelude::*;
use bevy_rand::global::GlobalRng;
use bevy_rand::prelude::WyRand;
use rand::RngExt;

use crate::components::*;
use crate::resources::*;
use crate::settings::BOUNCE_UP_SPEED;
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

pub fn on_bounced_patient(
    event: On<BouncedEvent>,
    mut patients: Query<&mut Velocity, With<Patient>>,
    human_player: Query<&Human>,
    sound_effect: Res<SoundEffect>,
    mut commands: Commands,
) {
    if let Ok(mut velocity) = patients.get_mut(event.patient) {
        if velocity.0.y.signum() == 1.0 {
            println!("Bounce it up!");
            if let Ok(_human) = human_player.get(event.bouncer) {
                velocity.0.y *= FALL_SPEED * BOUNCE_UP_SPEED;
                commands.trigger(PlayerBounceEvent);
                commands.spawn((
                    AudioPlayer::new(sound_effect.player_sound.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            } else {
                velocity.0.y *= FALL_SPEED * BOUNCE_UP_SPEED / 4.0;

                commands.spawn((
                    AudioPlayer::new(sound_effect.agent_sound.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}

pub fn another_fell_through(_event: On<FellThrough>, mut score: ResMut<Score>) {
    score.fell_through += 1;
    score.capacity -= 1;
}

pub fn another_helped(_event: On<PlayerBounceEvent>, mut score: ResMut<Score>) {
    score.helped += 1;
    score.capacity += 1;
}

pub fn check_game_over(
    _event: On<FellThrough>,
    score: ResMut<Score>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if score.capacity <= 0 {
        println!("Capacity {} - game over!", score.capacity);
        next_state.set(AppState::GameOver);
    }
}

pub fn add_another_patient(
    _event: On<AddAnotherPatientEvent>,
    commands: Commands,
    rng: Single<&mut WyRand, With<GlobalRng>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_patient(commands, rng, asset_server, window, texture_atlas_layouts);
}

pub fn possibly_add_agent(
    _event: On<BouncedEvent>,
    agents: Query<&Agent>,
    commands: Commands,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if should_add_agent(score.helped, agents.count()) {
        spawn_agent(commands, asset_server, window, texture_atlas_layouts);
    }
}

fn should_add_agent(score: isize, agents_count: usize) -> bool {
    if score > 0 {
        if (score as u32).is_power_of_two() && score > 1 {
            println!("Score: {}: Should add an agent?", score);
            let log_score = score.ilog2() as isize;
            println!("{} agents count vs {} log_score", agents_count, log_score);
            if agents_count < (log_score as usize) {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_add_agents() {
        assert!(should_add_agent(2, 0));
        assert!(should_add_agent(4, 1));
        assert!(should_add_agent(8, 2));
    }

    #[test]
    fn no_add_agents() {
        assert!(!should_add_agent(3, 0));
        assert!(!should_add_agent(3, 1));
        assert!(!should_add_agent(4, 2));
        assert!(!should_add_agent(6, 1));
        assert!(!should_add_agent(6, 2));
        assert!(!should_add_agent(8, 3));
    }
}

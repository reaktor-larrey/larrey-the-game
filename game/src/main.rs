use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{components::*, observers::*, resources::*, systems::*};

mod components;
mod js;
mod observers;
mod resources;
mod settings;
mod systems;

fn main() {
    #[cfg(not(target_family = "wasm"))]
    run_game(None);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn start_game_with_session(session_id: String) {
    use web_sys::console;
    console::log_1(&format!("Started game with session ID {}", session_id).into());
    run_game(Some(session_id));
}

fn run_game(session_id: Option<String>) {
    let seed: u64 = 123;
    let mut app = App::new();

    // #[cfg(debug_assertions)]
    // {
    //     use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    //     // app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    //     app.add_plugins(LogDiagnosticsPlugin::default());
    // }

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".into()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                #[cfg(target_family = "wasm")]
                file_path: "/assets/".into(),
                ..default()
            }),
    )
    .insert_resource(GameSession { id: session_id })
    .insert_resource(ClearColor(Color::srgb_u8(35, 50, 52)))
    .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
    .init_state::<AppState>()
    .insert_resource(Score {
        fell_through: 0,
        helped: 0,
        capacity: 1,
    })
    .init_resource::<WaveTimerResource>()
    .add_systems(
        Startup,
        (
            check_for_session,
            spawn_sound_effect,
            spawn_patient,
            spawn_player_paddle,
            spawn_gutters,
            spawn_scoreboard,
            spawn_camera,
        )
            .chain(),
    )
    .add_systems(Update, run_animations)
    .add_systems(
        FixedUpdate,
        (
            move_ball,
            apply_gravity,
            handle_collisions,
            handle_player_input,
            move_paddles,
            move_agents,
            constrain_paddle_position,
            project_positions,
            update_scoreboard,
            detect_fell_through,
            tick_wave_timer,
        )
            .run_if(in_state(AppState::Playing))
            .chain(),
    )
    .add_systems(OnEnter(AppState::GameOver), end_game)
    .add_observer(on_bounced_patient)
    .add_observer(reset_patient)
    .add_observer(another_fell_through)
    .add_observer(check_game_over)
    .add_observer(another_helped)
    .add_observer(add_another_patient)
    .add_observer(possibly_add_agent)
    .run();
}

use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::{components::*, observers::*, resources::*, systems::*};

mod components;
mod observers;
mod resources;
mod settings;
mod systems;

fn main() {
    let seed: u64 = 123;
    let mut app = App::new();

    // #[cfg(debug_assertions)]
    // {
    //     use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    //     app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    //     app.add_plugins(LogDiagnosticsPlugin::default());
    // }

    app.add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
        .insert_resource(Score { fell_through: 0 })
        .init_resource::<WaveTimerResource>()
        .add_systems(
            Startup,
            (
                spawn_patient,
                spawn_player_paddle,
                spawn_gutters,
                spawn_scoreboard,
                spawn_camera,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                move_ball,
                apply_gravity,
                handle_collisions,
                handle_player_bump_ball,
                handle_player_input,
                move_paddles,
                move_agents,
                constrain_paddle_position,
                project_positions,
                update_scoreboard,
                detect_fell_through,
                tick_wave_timer,
            )
                .chain(),
        )
        .add_observer(reset_patient)
        .add_observer(minus_one)
        .add_observer(plus_one)
        .add_observer(add_another_patient)
        .add_observer(possibly_add_agent)
        .run();
}

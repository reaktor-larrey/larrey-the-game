use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::systems::{startup::*, update::*};

mod components;
mod entities;
mod resources;
mod systems;

fn main() {
    let seed: u64 = 123;
    let mut app = App::new();

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    app.add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
        .add_systems(
            Startup,
            (
                spawn_ball,
                spawn_paddles,
                // spawn_gutters,
                spawn_scoreboard,
                spawn_camera,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                move_ball,
                handle_collisions,
                handle_player_input,
                move_paddles,
                constrain_paddle_position,
                project_positions,
                detect_fell_through,
            )
                .chain(),
        )
        .add_observer(reset_ball)
        .run();
}

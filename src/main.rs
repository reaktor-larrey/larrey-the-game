use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};

use crate::{
    resources::Score,
    systems::{startup::*, update::*},
};

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
        .insert_resource(Score { fell_through: 0 })
        .add_systems(
            Startup,
            (
                spawn_ball,
                spawn_paddles,
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
                handle_player_input,
                move_paddles,
                constrain_paddle_position,
                project_positions,
                update_scoreboard,
                detect_fell_through,
            )
                .chain(),
        )
        .add_observer(reset_ball)
        .add_observer(update_score)
        .run();
}

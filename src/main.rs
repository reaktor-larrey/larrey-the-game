use bevy::prelude::*;

use crate::systems::{startup::*, update::*};

mod components;
mod entities;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_ball, spawn_paddles, spawn_gutters, spawn_camera),
        )
        .add_systems(
            FixedUpdate,
            (
                move_ball,
                handle_collisions,
                handle_player_input,
                move_paddles,
                project_positions,
            )
                .chain(),
        )
        .run();
}

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
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
        .insert_resource(Score {
            human: 0,
            computer: 0,
        })
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
                handle_collisions,
                handle_player_input,
                move_ai,
                move_paddles,
                constrain_paddle_position,
                project_positions,
                detect_goal,
                update_scoreboard,
            )
                .chain(),
        )
        .add_observer(reset_ball)
        .add_observer(update_score)
        .run();
}

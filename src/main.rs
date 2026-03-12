use bevy::prelude::*;

use crate::{
    resources::Score,
    systems::{startup::*, update::*},
};

mod components;
mod entities;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score {
            human: 0,
            computer: 0,
        })
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
                constrain_paddle_position,
                project_positions,
                detect_goal,
            )
                .chain(),
        )
        .add_observer(reset_ball)
        .add_observer(update_score)
        .run();
}

use crate::{components::*, settings::BALL_SIZE};
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
    Sprite
)]
pub struct Agent;

pub fn spawn_agent(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let texture_handle = asset_server.load("agent-w-arms.png");

    commands.spawn((
        Agent,
        Paddle,
        Sprite {
            image: texture_handle,
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
            ..default()
        },
        Velocity(Vec2::new(PADDLE_SPEED, 0.)),
        Position(Vec2::new(0., -window.resolution.height() / 2.0 + BALL_SIZE)),
    ));
}

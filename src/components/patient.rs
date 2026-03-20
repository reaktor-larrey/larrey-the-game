use bevy::prelude::*;
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use rand::Rng;

use crate::components::*;

pub const BALL_SIZE: f32 = 64.0;
pub const FALL_SPEED: f32 = 1.5;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
    Sprite,

)]
pub struct Patient;

pub fn spawn_patient(
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let texture_handle = {
        let coin_toss = rng.next_u32() % 2 == 0;
        if coin_toss {
            asset_server.load("patient-m.png")
        } else {
            asset_server.load("patient-f.png")
        }
    };

    commands.spawn((
        Patient,
        Sprite {
            image: texture_handle,
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
            ..default()
        },
        Position(Vec2::new(0., window.resolution.height() / 2.0)),
    ));
}

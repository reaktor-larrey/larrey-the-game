use bevy::prelude::*;
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use rand::Rng;

use crate::components::{Collider, Position, Velocity};

pub const BALL_SIZE: f32 = 64.0;
pub const FALL_SPEED: f32 = 1.5;

enum Gender {
    Male,
    Female,
}

#[derive(Component)]
pub struct GenderPresentation {
    pub presenting_as: Gender,
}

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
    Sprite,
    GenderPresentation = GenderPresentation { presenting_as: Gender::Female }

)]
pub struct Patient;

pub fn spawn_patient(
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    asset_server: Res<AssetServer>,
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
    ));
}

use bevy::prelude::*;
use bevy_rand::{global::GlobalRng, prelude::WyRand};
use rand::Rng;

use crate::{components::*, settings::BALL_SIZE};

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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = {
        let coin_toss = rng.next_u32() % 2 == 0;
        if coin_toss {
            asset_server.load("patient-m-animation.png")
        } else {
            asset_server.load("patient-f-animation.png")
        }
    };

    // Sprite sheet stuff...
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation = AnimationConfig::new(1, 7, 15);

    commands.spawn((
        Patient,
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation.first_sprite_index,
            }),
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
            ..default()
        },
        Position(Vec2::new(
            0.,
            window.resolution.height() / 2.0 + PADDLE_HEIGHT * 6.0,
        )),
        animation,
    ));
}

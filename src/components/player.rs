use bevy::prelude::*;

use crate::components::*;

pub const PADDLE_WIDTH: f32 = 100.;
pub const PADDLE_HEIGHT: f32 = 10.0;
pub const PADDLE_SHAPE: Rectangle = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Collider = Collider(PADDLE_SHAPE),
    Sprite
)]
pub struct Paddle;

pub fn spawn_player_paddle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let player_position = Vec2::new(0., -half_window_size.y + padding);

    let texture_handle = asset_server.load("medical-animation.png");

    // Sprite sheet stuff...
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation = AnimationConfig::new(1, 7, 15);

    commands.spawn((
        Human,
        Paddle,
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: animation.first_sprite_index,
            }),
            custom_size: Some(Vec2::splat(PADDLE_WIDTH)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
            ..default()
        },
        Position(player_position),
        animation,
    ));
}

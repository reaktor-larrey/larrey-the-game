use crate::{
    components::*,
    settings::{AGENT_SPEED, BALL_SIZE},
};

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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("agent-animation.png");

    // Sprite sheet stuff...
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation = AnimationConfig::new(1, 7, 10);

    commands.spawn((
        Agent,
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
        Velocity(Vec2::new(AGENT_SPEED, 0.)),
        Position(Vec2::new(
            0.,
            -window.resolution.height() / 2.0 + PADDLE_WIDTH,
        )),
        animation,
    ));
}

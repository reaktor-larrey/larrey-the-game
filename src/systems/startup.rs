use bevy::prelude::*;

use crate::{components::*, resources::SoundEffect};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

pub fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let material = materials.add(GUTTER_COLOR);
    let padding = 20.;

    let gutter_shape = Rectangle::new(GUTTER_THICKNESS, window.resolution.height() * 2.0);
    let mesh = meshes.add(gutter_shape);

    let left_gutter_position = Vec2::new(-window.resolution.width() / 2. + padding, 0.);

    commands.spawn((
        Gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(left_gutter_position),
        Collider(gutter_shape),
    ));

    let right_gutter_position = Vec2::new(window.resolution.width() / 2. - padding, 0.);

    commands.spawn((
        Gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(right_gutter_position),
        Collider(gutter_shape),
    ));
}

pub fn spawn_scoreboard(mut commands: Commands) {
    // Create a container that will center everything
    let container = Node {
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    // Then add a container for the text
    let header = Node {
        width: px(200.),
        height: px(100.),
        ..default()
    };

    let font_size: f32 = 42.0;

    // The fell through score/count
    let fell_through_count = (
        FellThroughScore,
        Text::new("0"),
        TextFont::from_font_size(font_size),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.0),
            left: px(0.0),
            ..default()
        },
    );

    commands.spawn((
        container,
        children![(header, children![fell_through_count])],
    ));
}

pub fn spawn_sound_effect(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("bounce.mp3");
    commands.insert_resource(SoundEffect { handle });
}

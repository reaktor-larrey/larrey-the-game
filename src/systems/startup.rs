use bevy::prelude::*;

use crate::entities::{
    Human,
    ball::{BALL_COLOR, BALL_SHAPE, Ball},
    paddle::{PADDLE_COLOR, PADDLE_SHAPE, Paddle},
};

use crate::components::*;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);

    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let player_position = Vec2::new(0., -half_window_size.y + padding);
    commands.spawn((
        Human,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(player_position),
    ));
}

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

    let gutter_shape = Rectangle::new(GUTTER_THICKNESS, window.resolution.height());
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

    // The fell through score/count
    let fell_through_count = (
        FellThroughScore,
        Text::new("0"),
        TextFont::from_font_size(72.0),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.0),
            right: px(0.0),
            ..default()
        },
    );

    commands.spawn((
        container,
        children![(header, children![fell_through_count])],
    ));
}

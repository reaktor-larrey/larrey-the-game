use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;

fn spawn_ball(mut commands: Commands) {
    println!("Spawning ball...");
    commands.spawn(Ball);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

// System: project positions to transforms
fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(FixedUpdate, project_positions)
        .run();
}

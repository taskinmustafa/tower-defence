use bevy::prelude::*;
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.6, 0.2, 0.2)))
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(spawn_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

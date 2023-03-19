use bevy::prelude::*;
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_basic_scene)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    commands.spawn( //Plane
        PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Plane{size: 5.0,subdivisions: 8})),
            material: materials.add(Color::rgb(0.5, 0.1, 0.7).into()),
            ..default()
        }
    );
    commands.spawn( //Torus
        PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Torus{radius: 1.0,ring_radius: 0.4,subdivisions_segments: 32, subdivisions_sides: 16})),
            material: materials.add(Color::rgb(0.1, 0.25, 0.85).into()),
            transform: Transform::from_xyz(0.0, 0.4, 1.0),
            ..default()
        }
    );
    commands.spawn( //Point light
        PointLightBundle{
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        }
    );
}
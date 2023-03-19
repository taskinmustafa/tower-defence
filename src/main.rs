use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_glb_scene)
    .add_startup_system(spawn_point_lights)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(7.0, 5.0, -7.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn spawn_point_lights(
    mut commands: Commands,
){
    commands.spawn( //Point light
        PointLightBundle{
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(2.0, 5.0, -5.0),
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
            transform: Transform::from_xyz(1.0, 5.0, -1.0),
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
            transform: Transform::from_xyz(3.0, 5.0, -1.0),
            ..default()
        }
    );
}

fn spawn_glb_scene(
    mut commands: Commands,
    assets: Res<AssetServer>,
){
    let my_glb = assets.load("scene0.glb#Scene0");
    commands.spawn(SceneBundle{
        scene: my_glb,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}


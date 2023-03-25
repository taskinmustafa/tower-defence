use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::{time::Duration, f32::consts::PI};
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
    .add_system(tower_shooting)
    .add_system(bullet_despawn)
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
    })
    .insert( Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating)
    });

}

#[derive(Component)]
pub struct Tower{
    shooting_timer: Timer,
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>
) {
    for mut tower in &mut towers{
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = 
                Transform::from_xyz(1.0, 1.8, -0.1) //bullet spawn point
                .with_rotation(Quat::from_rotation_x(-PI/2.0));
            commands.
                spawn(
                    PbrBundle{
                        mesh: meshes.add(Mesh::from(shape::Capsule { radius: 0.1, depth: 0.3, latitudes: 4, longitudes: 8, rings: 8, uv_profile: shape::CapsuleUvProfile::Aspect })),
                        material: materials.add(Color::DARK_GRAY.into()),
                        transform: spawn_transform,
                        ..default()
                    }
                )
                .insert(
                    Lifetime{
                        timer: Timer::from_seconds(0.5, TimerMode::Once)
                    }
                );
        }
    }
    
}
#[derive(Component)]
pub struct Lifetime{
    timer: Timer,
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>
) {
    for (entity, mut bullet) in &mut bullets{
        bullet.timer.tick(time.delta());
        if bullet.timer.finished(){
            commands.entity(entity).despawn_recursive();
        }
    }
}
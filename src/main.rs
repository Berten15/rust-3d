use bevy::{prelude::*};
use bevy_inspector_egui::{WorldInspectorPlugin, WorldInspectorParams};

mod components;
mod physics;
mod minecart;
mod rails;
// mod asset_server;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()

        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "rust 3d".to_string(),
            resizable: false,
            ..default()
        })
    
        // Systems
        // .add_startup_system(asset_server::load)
        // .add_startup_system(minecart::spawn_minecart)
        .add_startup_system(rails::spawn_rails)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(physics::movement_system)
        .add_system(physics::acceleration_system)

        // Plugins
        .add_plugins(DefaultPlugins)

        // World Inspector
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(toggle_inspector)

        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(Name::new("Camera"));
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0})),
        material: materials.add(Color::rgb(0.65, 0.85, 0.9).into()),
        ..default()
    })
    .insert(Name::new("Ground"));


}

fn toggle_inspector(input: Res<Input<KeyCode>>, mut world_inspector_params: ResMut<WorldInspectorParams>) {
    if input.just_pressed(KeyCode::Escape) {
        world_inspector_params.enabled = !world_inspector_params.enabled;
    }
}


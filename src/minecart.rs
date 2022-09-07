use bevy::prelude::*;

use crate::components;

pub fn spawn_minecart(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0})),
        material: materials.add(Color::rgb(1.0, 0.65, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Name::new("Minecart"))
    .insert(components::Speed {vec: Vec3::ZERO});
}
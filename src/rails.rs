use std::fs;
use bevy::prelude::*;

// use crate::asset_server;

pub fn spawn_rails(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    
    // load assets
    let rail = assets.load("railroad.glb#Scene0");

    let contents = fs::read_to_string("railroad_layout.txt")
        .expect("Should have been able to read the file");
    
    for char in contents.chars(){
        println!("{}", char);
    }

    let mut x: f32 = -2.0;
    let mut z: f32 = -2.0;

    for char in contents.chars(){
        if char == '|' {
            commands.spawn_bundle(SceneBundle {
                scene: rail.clone(),
                // scene: assets.railroad_straight.clone(),
                transform: 
                    Transform::from_xyz(x, 0.0, z)
                    .with_scale(Vec3::splat(0.05))
                ,
                ..default()
            })
            .insert(Name::new("Rails"));
        }
        if char == '-' {
            commands.spawn_bundle(SceneBundle {
                scene: rail.clone(),
                // scene: assets.railroad_straight.clone(),
                transform: 
                    Transform::from_xyz(x, 0.0, z)
                    .with_rotation(Quat::from_rotation_y((90.0_f32).to_radians()))
                    .with_scale(Vec3::splat(0.05))
                ,
                ..default()
            })
            .insert(Name::new("Rails"));
        }
        x += 1.0;
        if char == '\n'{
            x = -2.0;
            z += 1.0;
        }
    }

    // commands.spawn_bundle(SceneBundle {
    //     scene: rail,
    //     // scene: assets.railroad_straight.clone(),
    //     transform: 
    //         Transform::from_xyz(0.0, 0.0, 0.0)
    //         .with_scale(Vec3::splat(0.1))
    //     ,
    //     ..default()
    // })
    // .insert(Name::new("Rails"));
}
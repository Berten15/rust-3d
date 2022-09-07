use bevy::prelude::{Query, Transform, Vec3};

use crate::components;

pub fn movement_system(mut movables: Query<(&components::Speed, &mut Transform)>) {
    for (speed, mut transform) in movables.iter_mut(){
        transform.translation += speed.vec/100.0;
    }
}

pub fn acceleration_system(mut speeds: Query<&mut components::Speed>) {
    for mut speed in speeds.iter_mut(){
        speed.vec += Vec3::new(0.01, 0.0, 0.0)
    }
}
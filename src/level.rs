use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::loading::MeshAssets;
use crate::{physics, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_floor));
    }
}

fn spawn_floor(mut commands: Commands, meshes: Res<MeshAssets>) {
    commands
        .spawn_bundle(ColorMesh2dBundle {
            material: meshes.floor.material.clone(),
            mesh: meshes.floor.mesh.clone().into(),
            ..default()
        })
        .insert(physics::Groups::level())
        .insert(meshes.floor.collider.clone())
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(0.0, -100.0, 0.0).with_scale(Vec3::new(1000.0, 15.0, 1.0)),
        ));
}

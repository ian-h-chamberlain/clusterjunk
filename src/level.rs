use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{physics, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_floor));
    }
}

fn spawn_floor(mut commands: Commands) {
    commands
        .spawn()
        .insert(physics::Groups::level())
        .insert(Collider::cuboid(500.0, 2.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -250.0, 0.0)));
}

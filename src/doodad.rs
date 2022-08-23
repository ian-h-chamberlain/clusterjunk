use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

pub struct DoodadPlugin;

impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::new(Duration::from_secs(2), true)))
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(prepare_meshes))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_doodads));
    }
}

struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Doodad;

struct DoodadAssets {
    square: DoodadAsset,
}

struct DoodadAsset {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

fn prepare_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square = {
        let mesh = meshes.add(shape::Cube::new(100.0).into());
        let material = materials.add(ColorMaterial {
            color: Color::BLUE,
            ..default()
        });
        DoodadAsset { mesh, material }
    };

    commands.insert_resource(DoodadAssets { square });
}

fn spawn_doodads(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<DoodadAssets>,
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        // TOOD check for collision before spawning it

        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.square.mesh.clone().into(),
                material: assets.square.material.clone(),
                transform: Transform::from_xyz(100.0, 0.0, 0.0),
                ..default()
            })
            .insert(Collider::cuboid(50.0, 50.0))
            .insert(RigidBody::Dynamic)
            .insert(Doodad);
    }
}

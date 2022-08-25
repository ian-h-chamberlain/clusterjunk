use std::time::Duration;

use bevy::{log, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{collision, GameState};

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
    collider: Collider,
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
        let collider = Collider::cuboid(50.0, 50.0);

        DoodadAsset {
            mesh,
            material,
            collider,
        }
    };

    commands.insert_resource(DoodadAssets { square });
}

fn spawn_doodads(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<DoodadAssets>,
    rapier_context: Res<RapierContext>,
    doodads: Query<Entity, With<Doodad>>,
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        let collider = assets.square.collider.clone();
        let shape_pos = Vec2::new(100.0, 0.0);
        let filter = QueryFilter::default();

        let mut can_spawn = true;
        rapier_context.intersections_with_shape(shape_pos, 0.0, &collider, filter, |entity| {
            if doodads.get(entity).is_ok() {
                can_spawn = false;
                false
            } else {
                true
            }
        });

        if !can_spawn {
            log::debug!("not spawning doodad at {shape_pos:?} that would collide");
            return;
        }

        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.square.mesh.clone().into(),
                material: assets.square.material.clone(),
                transform: Transform::from_translation(shape_pos.extend(0.0)),
                ..default()
            })
            .insert(collision::Groups::doodad())
            .insert(collider)
            .insert(RigidBody::Dynamic)
            .insert(Doodad);
    }
}

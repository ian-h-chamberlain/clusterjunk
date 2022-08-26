use std::time::Duration;

use bevy::{log, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::loading::MeshAssets;
use crate::physics;
use crate::GameState;

pub struct DoodadPlugin;

impl Plugin for DoodadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::new(Duration::from_secs(1), true)))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_doodads));
    }
}

struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Doodad;

fn spawn_doodads(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<MeshAssets>,
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
            log::info!("not spawning doodad at {shape_pos:?} that would collide");
            return;
        }

        commands
            .spawn_bundle(
                physics::ColliderBundle::from(&assets.square).with_transform(
                    Transform::from_translation(shape_pos.extend(10.0))
                        .with_scale(Vec3::splat(20.0)),
                ),
            )
            .insert(Doodad);
    }
}

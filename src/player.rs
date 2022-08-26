use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::actions::Actions;
use crate::doodad::Doodad;
use crate::loading::MeshAssets;
use crate::physics;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player)
                    .with_system(combine_with_doodads),
            );
    }
}

fn spawn_player(mut commands: Commands, meshes: Res<MeshAssets>) {
    commands
        .spawn_bundle(
            physics::ColliderBundle::from(&meshes.player).with_transform(
                Transform::from_translation(Vec3::new(0.0, 0.0, 10.0))
                    .with_scale(Vec3::splat(25.0)),
            ),
        )
        .insert(Player)
        .insert(Velocity::zero())
        .insert_bundle(physics::PlayerBundle::default());
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    const MAX_ANGULAR_SPEED: f32 = 30.0;
    const MAX_LINEAR_SPEED: f32 = 300.0;
    const ANGULAR_ACCEL: f32 = 75.0;

    if actions.player_movement.is_none() {
        return;
    }

    // TODO: control in the air seems necessary too, at some point

    let x_mov = actions.player_movement.unwrap().x * ANGULAR_ACCEL * time.delta_seconds();

    // NOTE: besides just pinning X movement, it might be cool to dynamically
    // lower the friction coefficient when we're trying to sanic-ball

    for mut player_vel in &mut player_query {
        // flip it so that left-arrow moves us left (rotates CCW)
        player_vel.angvel =
            (player_vel.angvel - x_mov).clamp(-MAX_ANGULAR_SPEED, MAX_ANGULAR_SPEED);

        player_vel.linvel.x = player_vel
            .linvel
            .x
            .clamp(-MAX_LINEAR_SPEED, MAX_LINEAR_SPEED);
    }
}

fn combine_with_doodads(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    actions: Res<Actions>,
    player: Query<
        (Entity, &GlobalTransform, &Handle<ColorMaterial>),
        (With<Player>, Without<Parent>),
    >,
    player_colliders: Query<(&GlobalTransform, &Collider), With<Player>>,
    mut doodads: Query<
        (&GlobalTransform, &mut Transform, &mut Handle<ColorMaterial>),
        (With<Doodad>, Without<Player>),
    >,
) {
    if !actions.combine {
        return;
    }

    let (root_player, player_transform, player_material) = player.single();

    let filter = QueryFilter::only_dynamic().groups(physics::CollideGroups::doodad().into());

    for (transform, collider) in &player_colliders {
        let transform = transform.compute_transform();
        // assume axis is always the same, since this is 2D
        let (_axis, shape_rot) = transform.rotation.to_axis_angle();
        let shape_pos = transform.translation.truncate();

        rapier_context.intersections_with_shape(shape_pos, shape_rot, collider, filter, |doodad| {
            if let Ok((doodad_global_transform, mut doodad_transform, mut material)) =
                doodads.get_mut(doodad)
            {
                commands.entity(root_player).add_child(doodad);

                commands
                    .entity(doodad)
                    // Doodad no longer moves on its own
                    .remove::<RigidBody>()
                    .remove::<CollisionGroups>()
                    // And should be treated as a part of the player
                    .remove::<Doodad>()
                    .insert(physics::CollideGroups::player())
                    .insert(Player);

                *material = player_material.clone();

                *doodad_transform = Transform::from_matrix(
                    player_transform.compute_matrix().inverse()
                        * doodad_global_transform.compute_matrix(),
                );
                // to prevent Z-fighting with other doodads, snap almost to the player Z
                // I would have thought that just setting it to -1.0 would work, but
                // apparently that moves it past the clip plane...
                // Perhaps this gets overwritten somehow in the propagation phase...
                doodad_transform.translation.z = player_transform.translation().z - 1.0;
            }
            // Match all intersections, not just the first one
            true
        });
    }
}

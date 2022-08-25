use crate::actions::Actions;
use crate::doodad::Doodad;
use crate::loading::TextureAssets;
use crate::{collision, GameState};
use bevy::{log, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_floor),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                .with_system(combine_with_doodads),
        );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_scale(Vec3::splat(0.2)),
            ..default()
        })
        .insert(collision::Groups::player())
        .insert(Collider::ball(130.0))
        .insert(Player)
        .insert(Restitution::coefficient(0.5))
        .insert(Friction::new(6.0))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero());
}

// TODO maybe this becomes a scene plugin or something
fn spawn_floor(mut commands: Commands) {
    commands
        .spawn()
        .insert(collision::Groups::level())
        .insert(Collider::cuboid(2000.0, 20.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    // TODO: consider a max linear vel instead of angular...
    const MAX_ANGULAR_SPEED: f32 = 9.0;
    const ANGULAR_ACCEL: f32 = 40.0;

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
    }
}

fn combine_with_doodads(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    actions: Res<Actions>,
    player_q: Query<(Entity, &GlobalTransform, &Collider), With<Player>>,
    mut doodads: Query<
        (&mut Transform, &GlobalTransform, &mut CollisionGroups),
        (With<Doodad>, Without<Player>),
    >,
) {
    if !actions.combine {
        return;
    }

    let filter = QueryFilter::default().groups(collision::Groups::player_interaction());

    for (player, player_transform, player_collider) in &player_q {
        let player_transform = player_transform.compute_transform();
        // assume axis is always correct since this is 2D
        let (_axis, angle) = player_transform.rotation.to_axis_angle();

        rapier_context.intersections_with_shape(
            player_transform.translation.truncate(),
            angle,
            player_collider,
            filter,
            |doodad| {
                if let Ok((mut transform, global_transform, mut collision_groups)) =
                    doodads.get_mut(doodad)
                {
                    log::info!("Adding a doodad {doodad:?}");
                    *collision_groups = collision::Groups::player();
                    commands.entity(doodad).remove::<RigidBody>().insert(Player);
                    commands.entity(player).add_child(doodad);

                    *transform = Transform::from_matrix(
                        player_transform.compute_matrix().inverse()
                            * global_transform.compute_matrix(),
                    );
                }

                true
            },
        );
    }
}

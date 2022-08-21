use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
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
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_scale(Vec3::splat(0.4)),
            ..Default::default()
        })
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

    let x_mov = actions.player_movement.unwrap().x * ANGULAR_ACCEL * time.delta_seconds();

    // NOTE: besides just pinning X movement, it might be cool to dynamically
    // lower the friction coefficient when we're trying to sanic-ball

    for mut player_vel in &mut player_query {
        // flip it so that left-arrow moves us left (rotates CCW)
        player_vel.angvel =
            (player_vel.angvel - x_mov).clamp(-MAX_ANGULAR_SPEED, MAX_ANGULAR_SPEED);
    }
}

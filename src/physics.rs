use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::loading::MeshAsset;

bitflags::bitflags! {
    pub struct CollideGroups: u32 {
       const PLAYER = 1 << 0;
       const DOODAD = 1 << 1;
       const LEVEL  = 1 << 2;
    }
}

impl CollideGroups {
    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER.bits(),
            filters: Self::LEVEL.bits(),
        }
    }

    pub fn doodad() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::DOODAD.bits(),
            filters: Self::DOODAD.bits() | Self::LEVEL.bits(),
        }
    }

    pub fn level() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::LEVEL.bits(),
            filters: Self::all().bits(),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub collision_groups: CollisionGroups,
    pub restitution: Restitution,
    pub friction: Friction,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            collision_groups: CollideGroups::player(),
            restitution: Restitution::coefficient(0.5),
            friction: Friction::new(5.0),
        }
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    #[bundle]
    pub mesh: ColorMesh2dBundle,
    pub collider: Collider,
    pub rigidbody: RigidBody,
}

impl From<&MeshAsset> for ColliderBundle {
    fn from(asset: &MeshAsset) -> Self {
        let mesh = asset.mesh.clone().into();
        let material = asset.material.clone();
        let collider = asset.collider.clone();

        Self {
            mesh: ColorMesh2dBundle {
                mesh,
                material,
                ..default()
            },
            collider,
            rigidbody: RigidBody::Dynamic,
        }
    }
}

impl ColliderBundle {
    pub fn with_transform(self, transform: Transform) -> Self {
        Self {
            mesh: ColorMesh2dBundle {
                transform,
                ..self.mesh
            },
            ..self
        }
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

bitflags::bitflags! {
    pub struct Groups: u32 {
       const PLAYER = 0b0001;
       const DOODAD = 0b0010;
       const LEVEL  = 0b0100;
    }
}

impl Groups {
    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER.bits(),
            filters: Self::LEVEL.bits(),
        }
    }

    pub fn player_interaction() -> InteractionGroups {
        InteractionGroups {
            // I'm not sure why, but this needs to have DOODAD as a member to work.
            memberships: Self::PLAYER.bits() | Self::DOODAD.bits(),
            filter: Self::DOODAD.bits(),
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
            collision_groups: Groups::player(),
            restitution: Restitution::coefficient(0.5),
            friction: Friction::new(6.0),
        }
    }
}

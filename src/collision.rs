use bevy_rapier2d::prelude::*;

bitflags::bitflags! {
    pub struct Groups: u32 {
       const PLAYER     = 0b001;
       const DOODAD    = 0b010;
       const LEVEL      = 0b100;
    }
}

impl Groups {
    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER.bits(),
            filters: Self::PLAYER.bits() | Self::LEVEL.bits(),
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

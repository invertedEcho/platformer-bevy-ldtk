use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct NextLevelOrb;

#[derive(Bundle, Default, LdtkEntity)]
pub struct NextLevelOrbBundle {
    next_level: NextLevelOrb,
}

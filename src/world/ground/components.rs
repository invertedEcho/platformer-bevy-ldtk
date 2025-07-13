use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Ground;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    ground: Ground,
}

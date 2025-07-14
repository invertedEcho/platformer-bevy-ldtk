use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Component, Default)]
pub struct Mushroom;

#[derive(Default, LdtkEntity, Bundle)]
#[ldtk_entity]
pub struct MushroomBundle {
    mushroom: Mushroom,
}

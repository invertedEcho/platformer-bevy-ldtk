use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

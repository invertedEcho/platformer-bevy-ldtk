use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Platform;

#[derive(Bundle, LdtkIntCell, Default)]
#[from_int_grid_cell]
pub struct PlatformBundle {
    platform: Platform,
}

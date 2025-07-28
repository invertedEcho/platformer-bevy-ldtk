use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct OneWayPlatform;

#[derive(Bundle, LdtkIntCell, Default)]
#[from_int_grid_cell]
pub struct OneWayPlatformBundle {
    one_way_platform: OneWayPlatform,
}

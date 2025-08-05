use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use components::OneWayPlatformBundle;
use systems::{handle_one_way_platform, spawn_platform_colliders};

pub mod components;
pub mod systems;

const PLATFORM_INT_GRID_CELL: i32 = 1;

// TODO: herochar_jump_down animation when going down from platform

pub struct OneWayPlatformPlugin;

impl Plugin for OneWayPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<OneWayPlatformBundle>(
            "Platforms",
            PLATFORM_INT_GRID_CELL,
        )
        .add_systems(Update, (spawn_platform_colliders, handle_one_way_platform));
    }
}

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use components::PlatformBundle;
use systems::{platform_player_collision_detection, spawn_platform_colliders};

pub mod components;
mod systems;

const PLATFORM_INT_GRID_CELL: i32 = 1;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<PlatformBundle>("Platforms", PLATFORM_INT_GRID_CELL)
            .add_systems(
                Update,
                (
                    spawn_platform_colliders,
                    platform_player_collision_detection,
                ),
            );
    }
}

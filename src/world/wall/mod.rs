use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use components::WallBundle;
use systems::spawn_wall_colliders;

pub mod components;
mod systems;

const WALLS_LAYER_IDENTIFIER: &str = "Walls";
const INT_GRID_WALL_CELL: i32 = 1;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<WallBundle>(
            WALLS_LAYER_IDENTIFIER,
            INT_GRID_WALL_CELL,
        )
        .add_systems(Update, spawn_wall_colliders);
    }
}

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use components::GroundBundle;
use systems::spawn_ground_colliders;

pub mod components;
mod systems;

const GROUND_LAYER_IDENTIFIER: &str = "Ground";
const INT_GRID_GROUND_CELL: i32 = 1;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<GroundBundle>(
            GROUND_LAYER_IDENTIFIER,
            INT_GRID_GROUND_CELL,
        )
        .add_systems(Update, spawn_ground_colliders);
    }
}

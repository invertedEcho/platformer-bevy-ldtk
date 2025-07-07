use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use components::WallBundle;
use systems::spawn_wall_colliders;

mod components;
mod systems;

pub struct WallsPlugin;

const INT_GRID_WALL_CELL: i32 = 1;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<WallBundle>(INT_GRID_WALL_CELL)
            .add_systems(Update, spawn_wall_colliders);
    }
}

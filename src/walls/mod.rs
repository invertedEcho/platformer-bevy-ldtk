use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use components::WallBundle;
use resources::LevelWalls;
use systems::cache_wall_locations;

mod components;
pub mod resources;
mod systems;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelWalls>()
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_systems(Update, cache_wall_locations);
    }
}

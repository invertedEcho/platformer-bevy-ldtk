use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
pub struct PlayerBundle {
    player: Player,
    #[grid_coords]
    grid_coords: GridCoords,
}

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player {
    pub is_on_platform: bool,
    pub is_jumping: bool,
    // idk about this, but it works
    pub is_on_jump_from_mushroom: bool,
}

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
pub struct PlayerBundle {
    player: Player,
    #[grid_coords]
    grid_coords: GridCoords,
}

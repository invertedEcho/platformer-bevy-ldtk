use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player {
    pub is_on_platform: bool,
    pub is_jumping: bool,
    // idk about this, but it works ->
    // TODO: investigate whether we can merge this into is_jumping
    pub is_on_jump_from_mushroom: bool,
    pub current_save_point: Option<Vec3>,
}

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
pub struct PlayerBundle {
    player: Player,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerDeadAnimationTimer(pub Timer);

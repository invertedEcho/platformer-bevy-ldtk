use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player {
    pub jumping: bool,
    pub current_save_point: Option<Vec3>,
    pub on_horizontal_moving_platform: bool,
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

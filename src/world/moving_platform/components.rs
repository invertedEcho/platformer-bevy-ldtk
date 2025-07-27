use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct MovingPlatform;

#[derive(Default, Bundle, LdtkEntity)]
pub struct MovingPlatformBundle {
    moving_platform: MovingPlatform,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

// TODO: Give better name
#[derive(Component)]
pub struct MoveDirection {
    pub upwards: bool,
    pub index: usize,
}

impl Default for MoveDirection {
    fn default() -> Self {
        MoveDirection {
            upwards: true,
            index: 0,
        }
    }
}

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

#[derive(PartialEq)]
pub enum Direction {
    HorizontalForward,
    HorizontalBackwards,
    VerticalUpwards,
    VerticalDownwards,
}

#[derive(Component)]
pub struct MovingPlatformInfo {
    pub direction: Direction,
    pub points_index: usize,
    pub points: Vec<IVec2>,
}

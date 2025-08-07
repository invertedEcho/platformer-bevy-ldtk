use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::ground_detection::components::GroundDetectionSensor;

#[derive(Debug, Default, PartialEq)]
pub enum PlayerDirection {
    #[default]
    Forward,
    Backwards,
}

#[derive(Debug, Default, PartialEq)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
    Jump,
    Dead,
}

#[derive(Debug, Default, Component)]
pub struct Player {
    pub current_save_point: Option<Vec3>,
    pub on_horizontal_moving_platform: bool,
    pub direction: PlayerDirection,
    pub state: PlayerState,
}

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
pub struct PlayerBundle {
    player: Player,
    ground_detection: GroundDetectionSensor,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerDeadAnimationTimer(pub Timer);

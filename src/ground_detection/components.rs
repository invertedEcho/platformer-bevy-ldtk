use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct GroundDetectionSensor {
    pub on_ground: bool,
}

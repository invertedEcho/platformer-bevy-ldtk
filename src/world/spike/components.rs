use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Spike;

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpikeBundle {
    spike: Spike,
}

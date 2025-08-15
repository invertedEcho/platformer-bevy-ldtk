use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(LdtkEntity, Bundle, Default)]
pub struct FallingSpikeBundle {
    falling_spike: FallingSpike,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Component, Default)]
pub struct FallingSpike;

#[derive(LdtkEntity, Bundle, Default)]
pub struct FallingSpikeSensorBundle {
    trigger_falling_spike: FallingSpikeSensor,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Component, Default)]
pub struct FallingSpikeSensor;

#[derive(Component)]
pub struct FallingSpikeDelayTimer {
    pub timer: Timer,
    // TODO: This kinda defeats the purpose of rust lol, could be non existing reference
    pub falling_spike_entity: Entity,
}

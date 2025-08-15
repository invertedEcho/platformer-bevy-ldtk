use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::world::falling_spike::{
    components::{FallingSpikeBundle, FallingSpikeSensorBundle},
    systems::{
        handle_falling_spike_delay_timers, handle_falling_spike_sensor_collision,
        process_falling_spike_sensors, process_falling_spikes,
    },
};

mod components;
mod systems;

pub struct FallingSpikePlugin;

impl Plugin for FallingSpikePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<FallingSpikeBundle>("Falling_Spike")
            .register_ldtk_entity::<FallingSpikeSensorBundle>("Falling_Spike_Sensor")
            .add_systems(
                Update,
                (
                    process_falling_spikes,
                    process_falling_spike_sensors,
                    handle_falling_spike_sensor_collision,
                    handle_falling_spike_delay_timers,
                ),
            );
    }
}

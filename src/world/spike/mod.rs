use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::SpikeBundle;
use systems::{detect_player_collide_with_spike, process_spikes};

mod components;
mod systems;

const SPIKE_ENTITY_IDENTIFIER: &str = "Spike";

pub const SPIKE_SPRITE_PATH: &str = "miscellaneous sprites/spikes.png";

pub struct SpikePlugin;

impl Plugin for SpikePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SpikeBundle>(SPIKE_ENTITY_IDENTIFIER)
            .add_systems(Update, (process_spikes, detect_player_collide_with_spike));
    }
}

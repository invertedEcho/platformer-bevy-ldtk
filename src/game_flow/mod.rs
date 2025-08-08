use crate::{
    common::systems::animate_generic_sprite,
    game_flow::next_level_orb::components::NextLevelOrbBundle, state::GameState,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use next_level_orb::{
    components::NextLevelOrb,
    systems::{detect_player_next_level_orb_collision, process_next_level_orbs},
};

mod next_level_orb;

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<NextLevelOrbBundle>("Next_Level")
            .add_systems(
                Update,
                (
                    process_next_level_orbs,
                    animate_generic_sprite::<NextLevelOrb>,
                    detect_player_next_level_orb_collision,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

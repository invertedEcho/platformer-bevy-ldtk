use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::{Player, PlayerBundle};
use states::PlayerMovementType;
use systems::{
    log_player_velocity, player_movement, set_backwards_idle_sprite,
    set_backwards_player_run_sprite, set_forward_idle_player_sprite, set_forward_player_run_sprite,
    setup_player,
};

use crate::common::systems::animate_generic_sprite;

pub mod components;
mod states;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerMovementType>()
            .register_ldtk_entity_for_layer::<PlayerBundle>("Player", "Player")
            .add_systems(
                Update,
                (
                    setup_player,
                    animate_generic_sprite::<Player>,
                    player_movement,
                    log_player_velocity,
                ),
            )
            .add_systems(
                OnEnter(PlayerMovementType::ForwardRun),
                set_forward_player_run_sprite,
            )
            .add_systems(
                OnEnter(PlayerMovementType::ForwardIdle),
                set_forward_idle_player_sprite,
            )
            .add_systems(
                OnEnter(PlayerMovementType::BackwardsRun),
                set_backwards_player_run_sprite,
            )
            .add_systems(
                OnEnter(PlayerMovementType::BackwardsIdle),
                set_backwards_idle_sprite,
            );
    }
}

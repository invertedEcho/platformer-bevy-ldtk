use crate::player::movement::systems::player_movement;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::{Player, PlayerBundle};
use events::PlayerDeadEvent;
use movement::states::PlayerMovementType;
use physics::systems::player_on_ground_detection;
use states::PlayerState;
use systems::{
    handle_player_dead_event, handle_player_state_enter_alive, player_debug_line_follow_player,
    setup_player, tick_player_dead_animation_timer,
};
use visual::systems::{
    set_backwards_idle_sprite, set_backwards_player_run_sprite, set_forward_idle_player_sprite,
    set_forward_player_run_sprite,
};

use crate::common::systems::animate_generic_sprite;

pub mod components;
pub mod events;
pub mod movement;
mod physics;
pub mod states;
mod systems;
mod visual;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDeadEvent>()
            .init_state::<PlayerMovementType>()
            .init_state::<PlayerState>()
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                Update,
                (
                    setup_player,
                    animate_generic_sprite::<Player>,
                    player_movement.run_if(in_state(PlayerState::Alive)),
                    player_on_ground_detection,
                    handle_player_dead_event,
                    tick_player_dead_animation_timer,
                    player_debug_line_follow_player,
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
            )
            .add_systems(OnEnter(PlayerState::Alive), handle_player_state_enter_alive);
    }
}

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::PlayerBundle;
use states::PlayerMovementType;
use systems::{
    animate_sprite, player_movement, set_backwards_idle_sprite, set_backwards_player_run_sprite,
    set_forward_idle_player_sprite, set_forward_player_run_sprite, setup_player,
};

pub mod components;
mod states;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerMovementType>()
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (setup_player, animate_sprite, player_movement))
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

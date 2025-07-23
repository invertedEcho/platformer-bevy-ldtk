use crate::player::heart::resources::PlayerHeartResource;
use crate::player::movement::systems::player_movement;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::{Player, PlayerBundle};
use movement::states::PlayerMovementType;
use physics::systems::player_on_ground_detection;
use systems::setup_player;
use visual::systems::{
    set_backwards_idle_sprite, set_backwards_player_run_sprite, set_forward_idle_player_sprite,
    set_forward_player_run_sprite,
};

use crate::common::systems::animate_generic_sprite;

pub mod components;
pub mod heart;
mod movement;
mod physics;
mod systems;
mod visual;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerMovementType>()
            .init_resource::<PlayerHeartResource>()
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                Update,
                (
                    setup_player,
                    animate_generic_sprite::<Player>,
                    player_movement,
                    player_on_ground_detection,
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

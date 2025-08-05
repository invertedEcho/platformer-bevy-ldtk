use crate::player::movement::systems::player_movement;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::{Player, PlayerBundle};
use physics::systems::player_on_ground_detection;
use systems::{setup_player, tick_player_dead_animation_timer};
use visual::systems::handle_player_change_visual;

use crate::common::systems::animate_generic_sprite;

pub mod components;
pub mod movement;
mod physics;
mod systems;
mod visual;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                Update,
                (
                    setup_player,
                    animate_generic_sprite::<Player>,
                    player_movement,
                    player_on_ground_detection,
                    tick_player_dead_animation_timer,
                    handle_player_change_visual,
                ),
            );
    }
}

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::PlayerBundle;
use systems::{animate_sprite, player_movement, process_player};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (process_player, animate_sprite, player_movement));
    }
}

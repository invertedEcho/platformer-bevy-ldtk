use bevy::prelude::*;
use systems::{despawn_main_menu, handle_interaction, spawn_main_menu};

use crate::state::GameState;

mod components;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(Update, handle_interaction);
    }
}

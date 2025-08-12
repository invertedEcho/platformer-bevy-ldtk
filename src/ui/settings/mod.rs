use bevy::prelude::*;
use systems::{despawn_settings_menu, handle_button_press, spawn_settings_menu};

use crate::state::GameState;

mod components;
mod systems;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), spawn_settings_menu)
            .add_systems(OnExit(GameState::Settings), despawn_settings_menu)
            .add_systems(Update, handle_button_press);
    }
}

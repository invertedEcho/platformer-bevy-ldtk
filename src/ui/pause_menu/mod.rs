use bevy::prelude::*;
use systems::{despawn_pause_menu, handle_escape_pause, spawn_pause_menu};

use crate::state::GameState;

mod components;
mod systems;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(Update, handle_escape_pause)
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
    }
}

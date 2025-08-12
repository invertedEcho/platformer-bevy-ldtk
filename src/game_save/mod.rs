use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use systems::load_game_save;
use utils::handle_game_save_text_timer;

use crate::INITIAL_LEVEL_IID;

mod components;
mod systems;
pub mod utils;

pub struct GameSavePlugin;

impl Plugin for GameSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_game_save)
            .add_systems(Update, handle_game_save_text_timer);
    }
}

const GAME_SAVE_FILE_PATH: &str = "game_save.json";

#[derive(Serialize, Deserialize)]
pub struct GameSave {
    pub level_iid: String,
    pub player_coins: i32,
}

impl Default for GameSave {
    fn default() -> Self {
        GameSave {
            level_iid: INITIAL_LEVEL_IID.to_string(),
            player_coins: 0,
        }
    }
}

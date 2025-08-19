use bevy::log::info;

use crate::game_save::{
    CURRENT_GAME_SAVE_VERSION, GameSave, OnlyGameSaveVersion, utils::update_game_save,
};

pub fn migrate_game_save(
    current_game_save_content: &mut String,
) -> Result<GameSave, serde_json::Error> {
    let maybe_only_game_save_version: Result<OnlyGameSaveVersion, serde_json::error::Error> =
        serde_json::from_str(&current_game_save_content);
    match maybe_only_game_save_version {
        Ok(result) => {
            if result.game_save_version == 0.1 && CURRENT_GAME_SAVE_VERSION == 0.2 {
                current_game_save_content.insert_str(
                    current_game_save_content.len() - 1,
                    ",\"grappling_hook_unlocked\": false",
                );
                let mut migrated_game_save: GameSave =
                    serde_json::from_str(current_game_save_content)?;
                migrated_game_save.game_save_version = CURRENT_GAME_SAVE_VERSION;
                update_game_save(&migrated_game_save);
                info!(
                    "Sucessfully migrated game save to a newer version!: {}",
                    current_game_save_content
                );
                return Ok(migrated_game_save);
            } else {
                panic!(
                    "No game save migration found for given parameters. User Game Save Version: {} Current Game Save Version: {}",
                    result.game_save_version, CURRENT_GAME_SAVE_VERSION
                )
            }
        }
        Err(error) => {
            // if this also fails, it means our game save file is corrupt.
            // TODO: This is too technical for the user. Integrate a more user friendly
            // logic
            panic!(
                "Your game save file is corrupt. Please consider/renaming your current game save: {}",
                error
            )
        }
    }
}

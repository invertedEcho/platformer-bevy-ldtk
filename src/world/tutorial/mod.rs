use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelSelection, app::LdtkEntityAppExt};
use components::{KeyboardTileBundle, TutorialTextBundle};
use systems::{change_keyboard_tiles, spawn_keyboard_tiles, spawn_text_for_tutorial_text};

use crate::LEVEL_IIDS;

mod components;
mod systems;

pub const TUTORIAL_TEXT_IDENTIFIER: &str = "Tutorial_Text";

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<TutorialTextBundle>(TUTORIAL_TEXT_IDENTIFIER)
            .register_ldtk_entity::<KeyboardTileBundle>("Keyboard_Tile")
            .add_systems(
                Update,
                (
                    spawn_text_for_tutorial_text,
                    spawn_keyboard_tiles,
                    change_keyboard_tiles,
                ), // .run_if(resource_equals(LevelSelection::iid(LEVEL_IIDS[0]))),
            );
    }
}

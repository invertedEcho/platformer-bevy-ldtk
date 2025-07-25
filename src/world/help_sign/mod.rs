use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::{HelpSignBundle, KeyboardTileBundle};
use systems::{change_keyboard_tiles, spawn_help_text_for_help_signs, spawn_keyboard_tiles};

mod components;
mod systems;

// TODO: I dont like the name "HelpSign"

pub struct HelpSignPlugin;

impl Plugin for HelpSignPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<HelpSignBundle>("Help_Sign")
            .register_ldtk_entity::<KeyboardTileBundle>("Keyboard_Tile")
            .add_systems(
                Update,
                (
                    spawn_help_text_for_help_signs,
                    spawn_keyboard_tiles,
                    change_keyboard_tiles,
                ),
            );
    }
}

use bevy::prelude::*;
use common::CommonUiPlugin;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;
use settings::SettingsPlugin;

pub mod common;
pub mod main_menu;
pub mod pause_menu;
pub mod settings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(SettingsPlugin)
            .add_plugins(CommonUiPlugin);
    }
}

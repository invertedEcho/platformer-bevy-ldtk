use bevy::prelude::*;

#[derive(Component)]
pub struct SettingsRoot;

#[derive(Component)]
pub struct SettingsButton {
    pub settings_button_type: SettingsButtonType,
}

#[derive(Debug)]
pub enum SettingsButtonType {
    ResetGameSave,
}

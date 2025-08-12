use bevy::prelude::*;

#[derive(Component)]
pub struct CommonUiButton {
    pub button_type: CommonButtonType,
}

pub enum CommonButtonType {
    BackToMainMenu,
}

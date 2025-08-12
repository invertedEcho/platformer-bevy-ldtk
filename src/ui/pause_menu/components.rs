use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub struct PauseMenuButton {
    pub pause_menu_button_type: PauseMenuButtonType,
}

pub enum PauseMenuButtonType {
    Resume,
}

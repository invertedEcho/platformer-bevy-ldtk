use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct MainMenuButton {
    pub main_menu_button_type: MainMenuButtonType,
}

#[derive(Debug)]
pub enum MainMenuButtonType {
    Play,
    Settings,
    Quit,
}

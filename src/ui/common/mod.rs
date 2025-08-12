use bevy::prelude::*;
use systems::{handle_button_interaction_hover, handle_common_button_press};

pub mod components;
mod systems;

pub struct CommonUiPlugin;

impl Plugin for CommonUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_common_button_press, handle_button_interaction_hover),
        );
    }
}

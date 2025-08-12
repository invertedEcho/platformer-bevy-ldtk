use bevy::prelude::*;
use systems::handle_common_button_press;

pub mod components;
mod systems;

pub struct CommonUiPlugin;

impl Plugin for CommonUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_common_button_press);
    }
}

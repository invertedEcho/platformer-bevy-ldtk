use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::HelpSignBundle;
use systems::spawn_help_text_for_help_signs;

mod components;
mod systems;

pub struct HelpSignPlugin;

impl Plugin for HelpSignPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<HelpSignBundle>("Help_Sign")
            .add_systems(Update, spawn_help_text_for_help_signs);
    }
}

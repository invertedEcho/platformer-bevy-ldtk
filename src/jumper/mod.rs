use bevy::prelude::*;
use systems::ground_detection_system;

pub mod components;
mod systems;

pub struct JumperPlugin;

impl Plugin for JumperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ground_detection_system);
    }
}

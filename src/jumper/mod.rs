use bevy::prelude::*;
use systems::player_on_ground_detection;

pub mod components;
mod systems;

pub struct JumperPlugin;

impl Plugin for JumperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_on_ground_detection);
    }
}

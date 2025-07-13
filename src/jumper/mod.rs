use bevy::prelude::*;
use systems::ground_collision_detection;

pub mod components;
mod systems;

pub struct JumperPlugin;

impl Plugin for JumperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ground_collision_detection);
    }
}

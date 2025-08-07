use bevy::prelude::*;
use systems::{detect_ground_collision, setup_ground_detection};

pub mod components;
mod systems;

pub struct GroundDetectionPlugin;

impl Plugin for GroundDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup_ground_detection, detect_ground_collision));
    }
}

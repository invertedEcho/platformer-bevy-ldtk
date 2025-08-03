use bevy::prelude::*;
use systems::{camera_follow_player_with_level_clamping, spawn_camera};

mod systems;

pub const CAMERA_SCALE: f32 = 0.4;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_player_with_level_clamping);
    }
}

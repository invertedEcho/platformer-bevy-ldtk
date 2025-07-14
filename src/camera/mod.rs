use bevy::prelude::*;
use systems::{camera_follow_player, spawn_camera};

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_player);
    }
}

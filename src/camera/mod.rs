use bevy::prelude::*;
use systems::camera_fit_inside_current_level;

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, camera_fit_inside_current_level);
    }
}

use bevy::prelude::*;

use crate::common::systems::detect_kill_player_collision_with_player;

pub mod components;
pub mod systems;

pub const NORMAL_ANIMATION_TIMER_DURATION: f32 = 0.1;

/// A plugin adding functionality that does not apply to a single module
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_kill_player_collision_with_player);
    }
}

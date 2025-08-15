use bevy::prelude::*;

use crate::world::falling_spike::FallingSpikePlugin;

pub mod falling_spike;
pub mod ground;
pub mod moving_platform;
pub mod mushroom;
pub mod one_way_platform;
pub mod save_point;
pub mod spike;
pub mod tutorial;
pub mod wall;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FallingSpikePlugin);
    }
}

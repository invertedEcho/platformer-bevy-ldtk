use bevy::prelude::*;
use systems::respawn_world;

mod systems;

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, respawn_world);
    }
}

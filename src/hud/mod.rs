use bevy::prelude::*;
use systems::{spawn_hud, update_coin_counter};

mod components;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_coin_counter);
    }
}

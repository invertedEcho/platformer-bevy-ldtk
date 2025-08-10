use bevy::prelude::*;
use systems::{spawn_hud, update_coin_counter};

use crate::state::GameState;

mod components;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hud)
            .add_systems(
                Update,
                (update_coin_counter).run_if(in_state(GameState::InGame)),
            );
    }
}

use bevy::prelude::*;
use systems::{spawn_hud, update_coin_counter, update_player_heart_count};

use crate::player::heart::resources::PlayerHeartResource;

mod components;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud).add_systems(
            Update,
            (
                update_coin_counter,
                update_player_heart_count.run_if(
                    resource_changed::<PlayerHeartResource>
                        .and(not(resource_added::<PlayerHeartResource>)),
                ),
            ),
        );
    }
}

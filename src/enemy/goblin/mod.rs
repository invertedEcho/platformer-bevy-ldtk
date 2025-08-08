use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::{Goblin, GoblinBundle};
use systems::{
    goblin_follow_player, handle_enemy_triggered_timer, handle_goblin_triggered_event,
    setup_goblins, update_goblin_sprite_if_changed,
};

use crate::{
    common::{components::TextureAtlasIndices, systems::animate_generic_sprite},
    state::GameState,
};

mod components;
mod systems;

const GOBLIN_IDLE_ANIM_STRIP_PATH: &str = "enemies/goblin/goblin_idle_anim_strip_4.png";
const GOBLIN_IDLE_ANIM_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 3 };

const GOBLIN_RUN_ANIM_STRIP_PATH: &str = "enemies/goblin/goblin_run_anim_strip_6.png";
const GOBLIN_RUN_ANIM_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 5 };

pub struct GoblinPlugin;

impl Plugin for GoblinPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<GoblinBundle>("Goblin")
            .add_systems(
                Update,
                (
                    setup_goblins,
                    animate_generic_sprite::<Goblin>,
                    goblin_follow_player,
                    update_goblin_sprite_if_changed,
                    handle_goblin_triggered_event,
                    handle_enemy_triggered_timer,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

use bevy::prelude::*;
use components::EnemyTriggered;
use events::EnemyTriggeredEvent;
use goblin::GoblinPlugin;
use slime::SlimePlugin;
use systems::{detect_enemy_collision_with_player, keep_enemy_triggered_above_enemy_head};

use crate::{
    common::{components::TextureAtlasIndices, systems::animate_generic_sprite},
    state::GameState,
};

pub mod components;
pub mod events;
pub mod goblin;
pub mod slime;
pub mod systems;

pub const ENEMY_EXCLAMATION_MARK_ANIM_STRIP_PATH: &str =
    "miscellaneous sprites/exclamation_mark_anim_strip_4.png";
pub const ENEMY_EXCLAMATION_MARK_ANIM_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 3 };

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyTriggeredEvent>()
            .add_plugins((SlimePlugin, GoblinPlugin))
            .add_systems(
                Update,
                (
                    detect_enemy_collision_with_player,
                    animate_generic_sprite::<EnemyTriggered>,
                    keep_enemy_triggered_above_enemy_head,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

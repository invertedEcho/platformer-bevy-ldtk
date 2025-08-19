use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    common::{components::TextureAtlasIndices, systems::animate_generic_sprite},
    world::loot_box::{
        components::{LootBox, LootBoxBundle},
        systems::{
            detect_player_collision_with_loot_box, handle_loot_box_change_visual,
            handle_loot_box_opening_timer, process_loot_boxes,
        },
    },
};

mod components;
mod systems;
mod utils;

const LOOT_BOX_CLOSED_WIDTH: f32 = 16.0;
const LOOT_BOX_CLOSED_HEIGHT: f32 = 8.0;

const LOOT_BOX_ANIM_STRIP_PATH: &str = "miscellaneous sprites/loot_box_anim_strip_12.png";
const LOOT_BOX_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 11 };

const LOOT_BOX_OPENING_ANIM_STRIP_PATH: &str =
    "miscellaneous sprites/loot_box_anim_opening_strip_6.png";
const LOOT_BOX_OPENING_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 5 };

const LOOT_BOX_OPEN_WIDTH: u32 = 16;
const LOOT_BOX_OPEN_HEIGHT: u32 = 10;

const LOOT_BOX_OPEN_SPRITE_PATH: &str = "miscellaneous sprites/loot_box_open.png";

#[derive(PartialEq)]
pub enum Loot {
    GrapplingHook,
}

pub struct LootBoxPlugin;

impl Plugin for LootBoxPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<LootBoxBundle>("Loot_Box")
            .add_systems(
                Update,
                (
                    process_loot_boxes,
                    animate_generic_sprite::<LootBox>,
                    detect_player_collision_with_loot_box,
                    handle_loot_box_change_visual,
                    handle_loot_box_opening_timer,
                ),
            );
    }
}

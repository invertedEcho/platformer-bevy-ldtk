use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    common::{components::TextureAtlasIndices, systems::animate_generic_sprite},
    world::loot_box::{
        components::{LootBox, LootBoxBundle},
        systems::process_loot_boxes,
    },
};

mod components;
mod systems;

const LOOT_BOX_WIDTH: f32 = 16.0;
const LOOT_BOX_HEIGHT: f32 = 8.0;

const LOOT_BOX_ANIM_STRIP_PATH: &str = "miscellaneous sprites/loot_box_anim_strip_12.png";
const LOOT_BOX_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 11 };

pub struct LootBoxPlugin;

impl Plugin for LootBoxPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<LootBoxBundle>("Loot_Box")
            .add_systems(
                Update,
                (process_loot_boxes, animate_generic_sprite::<LootBox>),
            );
    }
}

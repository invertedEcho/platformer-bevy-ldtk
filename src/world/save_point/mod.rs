use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::{SavePoint, SavePointBundle};
use systems::{
    detect_player_collider_with_save_point, process_save_points, tick_saving_save_point_timer,
};

use crate::common::{components::TextureAtlasIndices, systems::animate_generic_sprite};

mod components;
mod systems;

const SAVE_POINT_ENTITY_IDENTIFIER: &str = "Save_Point";

const SAVE_POINT_ANIM_STRIP_PATH: &str = "miscellaneous sprites/save_point_anim_strip_9.png";
const SAVE_POINT_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 8 };

const SAVE_POINT_SAVING_ANIM_STRIP_PATH: &str =
    "miscellaneous sprites/save_point_saving_anim_strip_3.png";
const SAVE_POINT_SAVING_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 2 };

pub struct SavePointPlugin;

impl Plugin for SavePointPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SavePointBundle>(SAVE_POINT_ENTITY_IDENTIFIER)
            .add_systems(
                Update,
                (
                    process_save_points,
                    animate_generic_sprite::<SavePoint>,
                    detect_player_collider_with_save_point,
                    tick_saving_save_point_timer,
                ),
            );
    }
}

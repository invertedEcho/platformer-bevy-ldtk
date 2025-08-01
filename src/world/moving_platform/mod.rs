use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::MovingPlatformBundle;
use systems::{move_moving_platforms, process_moving_platforms};

pub mod components;
mod systems;

// Im not sure if moving platform and "normal" platform should be seperate plugins,
// but logic will differ a lot so i think its ok

// const MOVING_PLATFORM_TILE_WIDTH: i32 = 64;
const MOVING_PLATFORM_TILE_HEIGHT: i32 = 16;

const PLATFORM_SINGLE_MIDDLE_SPRITE_PATH: &str = "miscellaneous sprites/platform_single_middle.png";

const MOVING_PLATFORM_SPEED: f32 = 50.0;
const MOVING_PLATFORM_POINTS_IDENTIFIER: &str = "Moving_Platform_Points";

pub struct MovingPlatformPlugin;

impl Plugin for MovingPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<MovingPlatformBundle>("Moving_Platform")
            .add_systems(Update, (process_moving_platforms, move_moving_platforms));
    }
}

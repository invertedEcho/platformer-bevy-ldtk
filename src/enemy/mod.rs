use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::{Slime, SlimeBundle};
use systems::{detect_slime_collision_with_player, patrol_slimes, spawn_slimes};

use crate::common::systems::animate_generic_sprite;

mod components;
mod systems;

pub const ENEMY_SPEED: f32 = 100.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SlimeBundle>("Slime")
            .add_systems(
                Update,
                (
                    spawn_slimes,
                    animate_generic_sprite::<Slime>,
                    detect_slime_collision_with_player,
                    patrol_slimes,
                ),
            );
    }
}

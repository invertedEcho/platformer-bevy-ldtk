use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::{Mushroom, MushroomBundle};
use systems::{mushroom_collision_detection, spawn_mushroom_colliders};

use crate::{common::systems::animate_generic_sprite, state::GameState};

mod components;
mod systems;

pub struct MushroomPlugin;

impl Plugin for MushroomPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<MushroomBundle>("Mushroom")
            .add_systems(
                Update,
                (
                    spawn_mushroom_colliders,
                    animate_generic_sprite::<Mushroom>,
                    mushroom_collision_detection,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

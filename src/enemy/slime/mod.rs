use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::SlimeBundle;
use systems::{animate_and_move_collider_slime, patrol_slimes, setup_slimes};

use crate::state::GameState;

pub mod components;
mod systems;

pub const SLIME_SPEED: f32 = 50.0;

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SlimeBundle>("Slime")
            .add_systems(
                Update,
                (setup_slimes, animate_and_move_collider_slime, patrol_slimes)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

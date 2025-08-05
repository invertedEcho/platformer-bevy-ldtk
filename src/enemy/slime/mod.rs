use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::{Slime, SlimeBundle};
use systems::{patrol_slimes, setup_slimes};

use crate::common::systems::animate_generic_sprite;

mod components;
mod systems;

pub const SLIME_SPEED: f32 = 50.0;

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SlimeBundle>("Slime")
            .add_systems(
                Update,
                (setup_slimes, patrol_slimes, animate_generic_sprite::<Slime>),
            );
        // .add_systems(OnEnter(PlayerState::Respawning), stop_slime_patroling);
    }
}

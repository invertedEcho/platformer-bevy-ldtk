use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use components::SlimeBundle;
use systems::{animate_and_move_slime, detect_slime_collision_with_player, spawn_slimes};

mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_for_layer::<SlimeBundle>("Enemies", "Slime")
            .add_systems(Update, (spawn_slimes, animate_and_move_slime, detect_slime_collision_with_player));
    }
}

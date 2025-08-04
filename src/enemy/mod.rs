use bevy::prelude::*;
use goblin::GoblinPlugin;
use slime::SlimePlugin;
use systems::detect_enemy_collision_with_player;

pub mod components;
pub mod goblin;
pub mod slime;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SlimePlugin, GoblinPlugin))
            .add_systems(Update, detect_enemy_collision_with_player);
    }
}

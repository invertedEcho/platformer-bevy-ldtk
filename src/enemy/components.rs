use bevy::prelude::*;

/// The base component for any enemy. Every enemy-type should include this Component
#[derive(Component)]
pub struct Enemy;

/// A component which is spawned when an enemy gets triggered.
///
/// # Arguments
///
/// * `timer` The timer, indicating how long the exclamation mark should be visible
/// * `enemy_entity` The enemy entity this EnemyTriggered component belongs to
#[derive(Component)]
pub struct EnemyTriggered {
    pub timer: Timer,
    pub enemy_entity: Entity,
}

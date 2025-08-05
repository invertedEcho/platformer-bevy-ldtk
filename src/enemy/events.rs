use bevy::prelude::*;

/// An event that gets fired when an enemy is triggered, e.g. in range of player
///
/// # Arguments
/// * `enemy_entity` The enemy entity that was triggered
#[derive(Event)]
pub struct EnemyTriggeredEvent {
    pub enemy_entity: Entity,
}

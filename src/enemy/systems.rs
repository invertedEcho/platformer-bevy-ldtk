use bevy::prelude::*;

use crate::enemy::components::Enemy;

use super::components::EnemyTriggered;

pub fn keep_enemy_triggered_above_enemy_head(
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<EnemyTriggered>)>,
    enemy_triggered_query: Query<(&mut Transform, &EnemyTriggered)>,
) {
    for (mut enemy_triggered_transform, enemy_triggered) in enemy_triggered_query {
        let Some(enemy) = enemy_query
            .iter()
            .find(|(e, _)| *e == enemy_triggered.enemy_entity)
        else {
            // TODO: This kinda defeats the purpose of rust lol. do this another way, like setting
            // a property on enemy component or something triggered: bool
            warn!(
                "An EnemyTriggered contains a reference to a EnemyEntity which couldnt be found!"
            );
            continue;
        };

        enemy_triggered_transform.translation.x = enemy.1.translation.x;
    }
}

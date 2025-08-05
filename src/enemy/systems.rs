use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::components::{Player, PlayerState};

use super::components::{Enemy, EnemyTriggered};

pub fn detect_enemy_collision_with_player(
    mut collision_event_reader: EventReader<CollisionEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _) = *collision_event else {
            continue;
        };

        let is_collision_entities_enemy = enemy_query
            .iter()
            .any(|slime| slime == first_entity || slime == second_entity);
        if !is_collision_entities_enemy {
            continue;
        }

        let Some(mut player) = player_query
            .iter_mut()
            .find(|(entity, _)| *entity == first_entity || *entity == second_entity)
        else {
            continue;
        };

        player.1.state = PlayerState::Dead;
    }
}

pub fn keep_enemy_triggered_above_enemy_head(
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<EnemyTriggered>)>,
    enemy_triggered_query: Query<(&mut Transform, &EnemyTriggered)>,
) {
    for (mut enemy_triggered_transform, enemy_triggered) in enemy_triggered_query {
        let Some(enemy) = enemy_query
            .iter()
            .find(|(e, _)| *e == enemy_triggered.enemy_entity)
        else {
            warn!(
                "An EnemyTriggered contains a reference to a EnemyEntity which couldnt be found!"
            );
            continue;
        };

        enemy_triggered_transform.translation.x = enemy.1.translation.x;
    }
}

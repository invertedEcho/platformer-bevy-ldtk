use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::{components::Player, events::PlayerDeadEvent};

use super::components::Enemy;

pub fn detect_enemy_collision_with_player(
    mut collision_event_reader: EventReader<CollisionEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    mut player_dead_event_writer: EventWriter<PlayerDeadEvent>,
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

        let is_collision_entities_player = player_query
            .iter()
            .any(|player| player == first_entity || player == second_entity);
        if !is_collision_entities_player {
            continue;
        }

        player_dead_event_writer.write(PlayerDeadEvent);
    }
}

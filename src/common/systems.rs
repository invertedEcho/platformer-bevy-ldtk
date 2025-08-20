use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::components::KillsPlayer,
    player::components::{Player, PlayerState},
};

use super::components::{AnimationTimer, TextureAtlasIndices};

pub fn animate_generic_sprite<T: Component>(
    time: Res<Time>,
    query: Query<(&TextureAtlasIndices, &mut AnimationTimer, &mut Sprite), With<T>>,
) {
    for (indices, mut timer, mut sprite) in query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn detect_kill_player_collision_with_player(
    mut collision_event_reader: EventReader<CollisionEvent>,
    enemy_query: Query<Entity, With<KillsPlayer>>,
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

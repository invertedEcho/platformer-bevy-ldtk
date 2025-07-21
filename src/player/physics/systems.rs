use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::components::Player,
    world::{ground::components::Ground, platform::components::Platform},
};

pub fn player_on_ground_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut players: Query<(&mut Player, Entity), With<Player>>,
    ground_query: Query<Entity, Or<(With<Ground>, With<Platform>)>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                for (mut player, player_entity) in players.iter_mut() {
                    let collision_entities_is_ground = ground_query
                        .iter()
                        .any(|e| e == *first_entity || e == *second_entity);
                    if collision_entities_is_ground {
                        if *first_entity == player_entity || *second_entity == player_entity {
                            player.is_jumping = false;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

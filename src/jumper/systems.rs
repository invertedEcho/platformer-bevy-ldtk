use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::components::Player;

use crate::ground::components::Ground;

use super::components::Jumper;

pub fn ground_detection_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut players: Query<(&mut Jumper, Entity), With<Player>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _collision_event_flags) => {
                for (mut jumper, player_entity) in players.iter_mut() {
                    let ground_entity = ground_query.iter().find(|e| e == entity1 || e == entity2);
                    if let Some(_) = ground_entity {
                        if *entity1 == player_entity || *entity2 == player_entity {
                            jumper.is_jumping = false;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

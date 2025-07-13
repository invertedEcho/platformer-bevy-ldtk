use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::components::Player,
    world::{ground::components::Ground, platform::components::Platform},
};

use super::components::Jumper;

pub fn player_on_ground_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut players: Query<(&mut Jumper, Entity), With<Player>>,
    ground_query: Query<Entity, Or<(With<Ground>, With<Platform>)>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
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

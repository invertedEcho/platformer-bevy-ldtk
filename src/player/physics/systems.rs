use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{
        components::{Player, PlayerDirection},
        movement::states::PlayerMovementType,
    },
    world::{
        ground::components::Ground, moving_platform::components::MovingPlatform,
        one_way_platform::components::OneWayPlatform,
    },
};

pub fn player_on_ground_detection(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Player, Entity), With<Player>>,
    // TODO: could we just insert Ground in Platform too to only have to filter by Ground?
    ground_query: Query<Entity, Or<(With<Ground>, With<OneWayPlatform>, With<MovingPlatform>)>>,
    mut next_player_movement_type_state: ResMut<NextState<PlayerMovementType>>,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _) = *collision_event else {
            continue;
        };
        let is_collision_entities_player = player_query.iter().any(|(_, player_entity)| {
            player_entity == first_entity || player_entity == second_entity
        });
        if !is_collision_entities_player {
            continue;
        }

        let collision_entities_is_ground = ground_query
            .iter()
            .any(|ground| ground == first_entity || ground == second_entity);
        if !collision_entities_is_ground {
            continue;
        }

        let Ok((mut player, _)) = player_query.single_mut() else {
            continue;
        };
        player.jumping = false;
        if player.direction == PlayerDirection::Forward {
            next_player_movement_type_state.set(PlayerMovementType::ForwardIdle);
        } else if player.direction == PlayerDirection::Backwards {
            next_player_movement_type_state.set(PlayerMovementType::BackwardsIdle);
        }
    }
}

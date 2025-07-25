use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    player::{components::Player, events::PlayerDeadEvent},
};

use super::{SPIKE_SPRITE_PATH, components::Spike};

pub fn process_spikes(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    spike_query: Query<Entity, Added<Spike>>,
) {
    for spike in spike_query {
        commands.entity(spike).insert((
            Sprite {
                image: asset_server.load(SPIKE_SPRITE_PATH),
                flip_y: true,
                ..default()
            },
            Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

pub fn detect_player_collide_with_spike(
    mut colllision_event_reader: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    spike_query: Query<Entity, With<Spike>>,
    mut player_dead_event_writer: EventWriter<PlayerDeadEvent>,
) {
    for collision_event in colllision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _) = *collision_event else {
            continue;
        };

        let is_entities_spike = spike_query
            .iter()
            .any(|spike| spike == first_entity || spike == second_entity);
        if !is_entities_spike {
            continue;
        }

        let is_entities_player = player_query
            .iter()
            .any(|player| player == first_entity || player == second_entity);
        if !is_entities_player {
            continue;
        }
        player_dead_event_writer.write(PlayerDeadEvent);
    }
}

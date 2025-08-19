use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    common::components::KillsPlayer,
    player::components::Player,
    world::{
        falling_spike::components::{FallingSpike, FallingSpikeDelayTimer, FallingSpikeSensor},
        spike::SPIKE_SPRITE_PATH,
    },
};

pub fn process_falling_spikes(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<Entity, Added<FallingSpike>>,
) {
    for entity in query {
        commands.entity(entity).insert((
            Sprite {
                image: asset_server.load(SPIKE_SPRITE_PATH),
                ..default()
            },
            Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
            ActiveEvents::COLLISION_EVENTS,
            RigidBody::Fixed,
            KillsPlayer,
        ));
    }
}

pub fn process_falling_spike_sensors(
    mut commands: Commands,
    query: Query<Entity, Added<FallingSpikeSensor>>,
) {
    for entity in query {
        commands.entity(entity).insert((
            Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

// TODO: delay each spike fall, by introducing active bool property on falling spike, and waiting
// the delay_sec specified in ldtk_field float.
pub fn handle_falling_spike_sensor_collision(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEvent>,
    falling_spike_sensor_query: Query<(Entity, &EntityInstance), With<FallingSpikeSensor>>,
    player_query: Query<Entity, With<Player>>,
    mut falling_spike_query: Query<(Entity, &EntityInstance), With<FallingSpike>>,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _flags) = collision_event else {
            continue;
        };

        let Some(trigger_falling) = falling_spike_sensor_query
            .iter()
            .find(|(e, _)| e == first_entity || e == second_entity)
        else {
            continue;
        };

        let is_entities_player = player_query
            .iter()
            .any(|e| e == *first_entity || e == *second_entity);
        if !is_entities_player {
            continue;
        }

        let trigger_group = trigger_falling
            .1
            .get_string_field("Target_Group")
            .expect("ldtk field correctly typed");
        let matching_falling_spikes: Vec<(Entity, &EntityInstance)> = falling_spike_query
            .iter_mut()
            .filter(|(_, entity_instance)| {
                entity_instance
                    .get_string_field("Target_Group")
                    .expect("ldtk field correctly typed")
                    == trigger_group
            })
            .collect();

        for (entity, entity_instance) in matching_falling_spikes {
            let maybe_delay_in_seconds = entity_instance
                .get_maybe_float_field("Delay_In_Seconds")
                .expect("Float field exists");

            match maybe_delay_in_seconds {
                Some(delay_in_seconds) => {
                    commands.entity(entity).insert(FallingSpikeDelayTimer {
                        timer: Timer::from_seconds(*delay_in_seconds, TimerMode::Once),
                        falling_spike_entity: entity,
                    });
                }
                None => {
                    commands.entity(entity).insert(RigidBody::Dynamic);
                }
            }
        }
    }
}

pub fn handle_falling_spike_delay_timers(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<&mut FallingSpikeDelayTimer>,
) {
    for mut falling_spike_delay_timer in query {
        let timer = &mut falling_spike_delay_timer.timer;
        timer.tick(time.delta());

        if timer.finished() {
            commands
                .entity(falling_spike_delay_timer.falling_spike_entity)
                .insert(RigidBody::Dynamic);
        }
    }
}

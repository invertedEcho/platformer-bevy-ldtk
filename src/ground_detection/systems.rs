use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    ground_detection::components::GroundSensor,
    world::{
        moving_platform::components::MovingPlatform, one_way_platform::components::OneWayPlatform,
        wall::components::Wall,
    },
};

use super::components::GroundDetection;

pub fn setup_ground_detection(
    mut commands: Commands,
    ground_detection_query: Query<Entity, Added<GroundDetection>>,
) {
    for ground_detection_entity in ground_detection_query {
        commands
            .entity(ground_detection_entity)
            .with_children(|builder| {
                builder.spawn_empty().insert((
                    ActiveEvents::COLLISION_EVENTS,
                    Collider::cuboid(1.0, 1.0),
                    Transform::from_xyz(0.0, -HALF_TILE_SIZE, 0.0),
                    GroundSensor {
                        ground_detection_entity,
                        intersecting_ground_entities: HashSet::new(),
                    },
                    Sensor,
                ));
            });
    }
}

pub fn detect_ground_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut ground_sensor_query: Query<&mut GroundSensor>,
    ground_query: Query<Entity, Or<(With<Wall>, With<OneWayPlatform>, With<MovingPlatform>)>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _flags) => {
                if ground_query.contains(*first_entity) {
                    if let Ok(mut ground_sensor) = ground_sensor_query.get_mut(*second_entity) {
                        ground_sensor
                            .intersecting_ground_entities
                            .insert(*first_entity);
                    }
                } else if ground_query.contains(*second_entity) {
                    if let Ok(mut ground_sensor) = ground_sensor_query.get_mut(*first_entity) {
                        ground_sensor
                            .intersecting_ground_entities
                            .insert(*second_entity);
                    }
                }
            }
            CollisionEvent::Stopped(first_entity, second_entity, _flags) => {
                if ground_query.contains(*first_entity) {
                    if let Ok(mut ground_sensor) = ground_sensor_query.get_mut(*second_entity) {
                        ground_sensor
                            .intersecting_ground_entities
                            .remove(first_entity);
                    }
                } else if ground_query.contains(*second_entity) {
                    if let Ok(mut ground_sensor) = ground_sensor_query.get_mut(*first_entity) {
                        ground_sensor
                            .intersecting_ground_entities
                            .remove(second_entity);
                    }
                }
            }
        }
    }
}

pub fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

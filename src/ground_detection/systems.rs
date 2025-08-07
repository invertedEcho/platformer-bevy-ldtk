use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{world::wall::components::Wall};

use super::components::GroundDetectionSensor;

pub fn setup_ground_detection(
    mut commands: Commands,
    ground_detection_query: Query<Entity, Added<GroundDetectionSensor>>,
) {
    for ground_detection in ground_detection_query {
        info!("ground detection entity: {}", ground_detection);
        // commands.entity(ground_detection).insert((
        //     ActiveEvents::COLLISION_EVENTS,
        //     Collider::cuboid(1.0, 1.0),
        //     Transform::from_xyz(0.0, -HALF_TILE_SIZE + 2., 0.0),
        //     Sensor,
        // ));
    }
}

pub fn detect_ground_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut ground_sensor_query: Query<&mut GroundDetectionSensor>,
    // TODO: should match anything thats relevant
    wall_query: Query<Entity, With<Wall>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _flags) => {
                if wall_query.contains(*first_entity) {
                    if let Ok(mut ground_detection_sensor) =
                        ground_sensor_query.get_mut(*second_entity)
                    {
                        info!("setting on ground to true");
                        ground_detection_sensor.on_ground = true;
                    }
                } else if wall_query.contains(*second_entity) {
                    if let Ok(mut ground_detection_sensor) =
                        ground_sensor_query.get_mut(*first_entity)
                    {
                        info!("setting on ground to true");
                        ground_detection_sensor.on_ground = true;
                    }
                }
            }
            CollisionEvent::Stopped(first_entity, second_entity, _flags) => {
                if wall_query.contains(*first_entity) {
                    if let Ok(mut ground_detection_sensor) =
                        ground_sensor_query.get_mut(*second_entity)
                    {
                        info!("setting on ground to false");
                        ground_detection_sensor.on_ground = false;
                    }
                } else if wall_query.contains(*second_entity) {
                    if let Ok(mut ground_detection_sensor) =
                        ground_sensor_query.get_mut(*first_entity)
                    {
                        info!("setting on ground to false");
                        ground_detection_sensor.on_ground = false;
                    }
                }
            }
        }
    }
}

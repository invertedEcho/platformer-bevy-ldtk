use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{HALF_TILE_SIZE, TILE_SIZE, player::components::Player, utils::preprocess_grid_coords};

use super::components::Platform;

pub fn spawn_platform_colliders(
    mut commands: Commands,
    platform_query: Query<(Entity, &GridCoords), Added<Platform>>,
) {
    // We already have the exact same logic for ground too, there is literally only one small
    // difference, we use "Platform" component instead of "Ground". Its fine for now but this can
    // definitely be improved
    let platform_grid_coords = platform_query
        .iter()
        .map(|(_, grid_coords)| grid_coords)
        .collect();
    let processed_platform_grid_coords = preprocess_grid_coords(platform_grid_coords);

    for (y_coordinate, x_coordinates_nested) in processed_platform_grid_coords {
        for x_coordinates in x_coordinates_nested {
            let start_from_collider_x = x_coordinates[0];
            let end_from_collider_x = *x_coordinates
                .iter()
                .last()
                .expect("Can get last x coordinate from array");
            let middle = (start_from_collider_x + end_from_collider_x) as f32 / 2.0;

            let cuboid_half_x = x_coordinates.len() as f32 * TILE_SIZE as f32 / 2.0;

            let world_x = (middle * TILE_SIZE as f32) + HALF_TILE_SIZE;
            let world_y = (y_coordinate as f32 * TILE_SIZE) + HALF_TILE_SIZE + HALF_TILE_SIZE / 2.0;

            commands.spawn((
                Transform {
                    translation: Vec3::new(world_x, world_y as f32, 0.0),
                    ..Default::default()
                },
                Collider::cuboid(cuboid_half_x, HALF_TILE_SIZE / 2.0),
                Platform,
                Friction::new(1.0),
                RigidBody::Fixed,
                ActiveEvents::COLLISION_EVENTS,
                Visibility::Hidden,
            ));
        }
    }
}

pub fn platform_player_collision_detection(
    platform_query: Query<Entity, With<Platform>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Player, Entity), With<Player>>,
) {
    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let collision_entities_is_platform = platform_query
                    .iter()
                    .any(|platform| platform == *first_entity || platform == *second_entity);
                if collision_entities_is_platform {
                    for (mut player, player_entity) in player_query.iter_mut() {
                        if *first_entity == player_entity || *second_entity == player_entity {
                            player.is_on_platform = true;
                            player.is_on_jump_from_mushroom = false;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(first_entity, second_entity, _) => {
                let collision_entities_is_platform = platform_query
                    .iter()
                    .any(|platform| platform == *first_entity || platform == *second_entity);
                if collision_entities_is_platform {
                    for (mut player, player_entity) in player_query.iter_mut() {
                        if *first_entity == player_entity || *second_entity == player_entity {
                            player.is_on_platform = false;
                        }
                    }
                }
            }
        }
    }
}

pub fn activate_platform_colliders_if_player_jumping_from_mushroom(
    mut commands: Commands,
    player_query: Query<(&Velocity, &Player), With<Player>>,
    platform_query: Query<Entity, With<Platform>>,
) {
    for (velocity, player) in player_query {
        if velocity.linvel.y < 0.0 && !player.is_on_platform && player.is_on_jump_from_mushroom {
            for platform in platform_query {
                commands.entity(platform).remove::<ColliderDisabled>();
            }
        }
    }
}

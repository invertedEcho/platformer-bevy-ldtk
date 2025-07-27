use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{na::Vector, prelude::*};

use crate::{
    HALF_TILE_SIZE, TILE_SIZE, player::components::Player, utils::preprocess_grid_coords,
    world::platform::components::PlatformCollidingWithPlayer,
};

use super::components::Platform;

#[derive(SystemParam)]
pub struct OneWayPlatformPhysicsHook;

impl BevyPhysicsHooks for OneWayPlatformPhysicsHook {
    // Note that we dont have to filter by anything, as only Platforms have `ActiveHooks::MODIFY_SOLVER_CONTACTS` inserted.
    // If that changes we will have to filter
    fn modify_solver_contacts(&self, contact: ContactModificationContextView) {
        contact
            .raw
            .update_as_oneway_platform(&Vector::y(), std::f32::consts::FRAC_2_PI);
    }
}

pub fn spawn_platform_colliders(
    mut commands: Commands,
    platform_query: Query<(Entity, &GridCoords), Added<Platform>>,
    level_query: Query<(Entity, &LevelIid)>,
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

            let (level_entity, _) = level_query.single().expect("can get current level");
            commands.entity(level_entity).with_children(|level| {
                level.spawn((
                    Transform {
                        translation: Vec3::new(world_x, world_y as f32, 0.0),
                        ..Default::default()
                    },
                    Collider::cuboid(cuboid_half_x, HALF_TILE_SIZE / 2.0),
                    Platform,
                    ActiveEvents::COLLISION_EVENTS,
                    Friction::new(1.0),
                    RigidBody::Fixed,
                    ActiveHooks::MODIFY_SOLVER_CONTACTS,
                ));
            });
        }
    }
}

pub fn platform_player_collision_detection(
    mut commands: Commands,
    mut platform_query: Query<Entity, With<Platform>>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    player_query: Query<&Velocity, With<Player>>,
) {
    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let Some(collided_platform) = platform_query
                    .iter_mut()
                    .find(|entity| entity == first_entity || entity == second_entity)
                else {
                    continue;
                };

                info!(
                    "player started colliding with a platform, inserting CollidingWithPlayer into colliding platform"
                );
                commands
                    .entity(collided_platform)
                    .insert(PlatformCollidingWithPlayer);
            }
            CollisionEvent::Stopped(first_entity, second_entity, _) => {
                let Some(collided_platform) = platform_query
                    .iter_mut()
                    .find(|entity| entity == first_entity || entity == second_entity)
                else {
                    continue;
                };
                let Ok(player) = player_query.single() else {
                    error!(
                        "No player found in collision_event from player with platform, this shouldnt be possible."
                    );
                    continue;
                };

                // we dont want to remove colliderdisabled when the collision stops because we fall
                // through the platform
                if player.linvel.y > 0.0 {
                    info!(
                        "player stopped colliding with a platform and doesnt have negative velocity.y, removing CollidingWithPlayer from colliding platform"
                    );
                    commands
                        .entity(collided_platform)
                        .remove::<PlatformCollidingWithPlayer>();
                    commands
                        .entity(collided_platform)
                        .remove::<ColliderDisabled>();
                }
            }
        }
    }
}

pub fn detect_player_under_platform(
    mut commands: Commands,
    platform_query: Query<
        (Entity, &Transform),
        (With<Platform>, With<PlatformCollidingWithPlayer>),
    >,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    for (platform_entity, platform_transform) in platform_query {
        if player_transform.translation.y < platform_transform.translation.y {
            info!(
                "Player under platform, removing PlatformCollidingWithPlayer and ColliderDisabled"
            );
            commands
                .entity(platform_entity)
                .remove::<ColliderDisabled>();
            commands
                .entity(platform_entity)
                .remove::<PlatformCollidingWithPlayer>();
        }
    }
}

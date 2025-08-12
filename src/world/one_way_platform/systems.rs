use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{HALF_TILE_SIZE, TILE_SIZE, player::components::Player, utils::preprocess_grid_coords};

use super::components::OneWayPlatform;

pub fn spawn_platform_colliders(
    mut commands: Commands,
    platform_query: Query<(Entity, &GridCoords), Added<OneWayPlatform>>,
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
                        translation: Vec3::new(world_x, world_y, 0.0),
                        ..default()
                    },
                    Collider::cuboid(cuboid_half_x, HALF_TILE_SIZE / 2.0),
                    OneWayPlatform,
                    RigidBody::Fixed,
                    ActiveEvents::COLLISION_EVENTS,
                ));
            });
        }
    }
}

/// TODO: This function checks all platforms
pub fn handle_one_way_platform(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    one_way_platform_query: Query<(Entity, &Transform), With<OneWayPlatform>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_translation = player_transform.translation;
    for (one_way_platform_entity, one_way_platform_transform) in one_way_platform_query {
        let one_way_platform_translation = one_way_platform_transform.translation;

        let player_under_platform = player_translation.y < one_way_platform_translation.y;

        let key_s_pressed =
            key_input.pressed(KeyCode::KeyS) && !key_input.just_released(KeyCode::KeyS);

        if player_under_platform || key_s_pressed {
            commands
                .entity(one_way_platform_entity)
                .insert(ColliderDisabled);
        } else {
            commands
                .entity(one_way_platform_entity)
                .remove::<ColliderDisabled>();
        }
    }
}

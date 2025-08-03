use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{HALF_TILE_SIZE, TILE_SIZE, utils::preprocess_grid_coords};

use super::components::Ground;

pub fn spawn_ground_colliders(
    mut commands: Commands,
    ground_query: Query<(Entity, &GridCoords), Added<Ground>>,
    level_query: Query<(Entity, &LevelIid)>,
) {
    let ground_grid_coords = ground_query
        .iter()
        .map(|(_, grid_coords)| grid_coords)
        .collect();
    let processed_wall_grid_coords = preprocess_grid_coords(ground_grid_coords);

    for (y_coordinate, x_coordinates_nested) in processed_wall_grid_coords {
        for x_coordinates in x_coordinates_nested {
            let start_from_collider_x = x_coordinates[0];
            let end_from_collider_x = *x_coordinates
                .iter()
                .last()
                .expect("Can get last x coordinate from array");
            let middle = (start_from_collider_x + end_from_collider_x) as f32 / 2.0;

            let cuboid_half_x = x_coordinates.len() as f32 * TILE_SIZE as f32 / 2.0;

            let world_x = (middle * TILE_SIZE) + HALF_TILE_SIZE;
            let world_y = (y_coordinate as f32 * TILE_SIZE) + HALF_TILE_SIZE;

            let (level_entity, _) = level_query.single().expect("can get current level");
            commands.entity(level_entity).with_children(|level| {
                level.spawn((
                    Transform {
                        translation: Vec3::new(world_x, world_y as f32, 0.0),
                        ..Default::default()
                    },
                    Collider::cuboid(cuboid_half_x, HALF_TILE_SIZE),
                    Ground,
                    ActiveEvents::COLLISION_EVENTS,
                    Friction::new(1.0),
                ));
            });
        }
    }
}

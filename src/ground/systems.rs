use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{TILE_SIZE, utils::preprocess_grid_coords};

use super::components::Ground;

pub fn spawn_ground_colliders(mut commands: Commands, grounds: Query<&GridCoords, Added<Ground>>) {
    let processed_wall_grid_coords = preprocess_grid_coords(grounds.iter().collect());

    for (y_coordinate, x_coordinates_nested) in processed_wall_grid_coords {
        for x_coordinates in x_coordinates_nested {
            let start_from_collider_x = x_coordinates[0];
            let end_from_collider_x = *x_coordinates
                .iter()
                .last()
                .expect("Can get last x coordinate from array");
            let middle = (start_from_collider_x + end_from_collider_x) as f32 / 2.0;

            let cuboid_half_x = x_coordinates.len() as f32 * TILE_SIZE as f32 / 2.0;
            let cuboid_half_y = (TILE_SIZE / 2) as f32;

            let world_x = (middle * TILE_SIZE as f32) + (TILE_SIZE / 2) as f32;
            let world_y = ((y_coordinate * TILE_SIZE) as f32) + (TILE_SIZE / 2) as f32;

            commands.spawn((
                Transform {
                    translation: Vec3::new(world_x, world_y as f32, 0.0),
                    ..Default::default()
                },
                Collider::cuboid(cuboid_half_x, cuboid_half_y),
                Ground,
                Friction::new(1.0),
                RigidBody::Fixed,
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}

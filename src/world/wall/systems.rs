use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{HALF_TILE_SIZE, TILE_SIZE};

use super::components::Wall;

// We could even further optimize this, by combining rows together, but its fine for now
fn preprocess_grid_coords(wall_grid_coords: &Vec<&GridCoords>) -> HashMap<i32, Vec<Vec<i32>>> {
    // Create map where key is unique y coordinate and value are all x coordinates of that y
    // coordinate
    let mut all_x_coords_of_y: HashMap<i32, Vec<i32>> = HashMap::new();
    for grid_coords in wall_grid_coords {
        let current_x = grid_coords.x;
        let current_y = grid_coords.y;

        all_x_coords_of_y
            .entry(current_y)
            .or_insert_with(Vec::new)
            .push(current_x);
    }

    // Now create another HashMap, where key is unique y coordinate and value are arrays of the x
    // coordinates. new array if there should be a gap, e.g. [[1, 2, 3], [6, 7, 8]]
    let mut splitted_x_coords_with_gaps_of_y: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

    for (y_coordinate, all_x_coordinates_of_y_coordinate) in all_x_coords_of_y {
        let mut current_nested_level = 0;
        for (index, current_x_coordinate) in all_x_coordinates_of_y_coordinate.iter().enumerate() {
            let next_item = if index == all_x_coordinates_of_y_coordinate.len() - 1 {
                None
            } else {
                Some(all_x_coordinates_of_y_coordinate[index + 1])
            };

            let root_new_array = splitted_x_coords_with_gaps_of_y
                .entry(y_coordinate)
                .or_insert_with(|| vec![Vec::new()]);

            root_new_array[current_nested_level].push(*current_x_coordinate);

            if let Some(next_item) = next_item {
                if next_item.abs_diff(*current_x_coordinate) > 1 {
                    current_nested_level += 1;
                    root_new_array.push(Vec::new());
                }
            }
        }
    }
    return splitted_x_coords_with_gaps_of_y;
}

pub fn spawn_wall_colliders(
    mut commands: Commands,
    walls: Query<&GridCoords, Added<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
) {
    let processed_wall_grid_coords = preprocess_grid_coords(&walls.iter().collect());

    for (y_coordinate, x_coordinates_nested) in processed_wall_grid_coords {
        for x_coordinates in x_coordinates_nested {
            let start_from_collider_x = x_coordinates[0];
            let end_from_collider_x = *x_coordinates
                .iter()
                .last()
                .expect("Can get last x coordinate");
            let middle = (start_from_collider_x + end_from_collider_x) as f32 / 2.0;

            let cuboid_half_x = x_coordinates.len() as f32 * TILE_SIZE as f32 / 2.0;

            let world_x = (middle * TILE_SIZE) + HALF_TILE_SIZE;
            let world_y = (y_coordinate as f32 * TILE_SIZE) + HALF_TILE_SIZE;

            let (level_entity, _) = level_query.single().expect("can get current level");
            commands.entity(level_entity).with_children(|level| {
                level.spawn((
                    Transform::from_xyz(world_x, world_y, 0.0),
                    Collider::cuboid(cuboid_half_x, HALF_TILE_SIZE),
                    Wall,
                    Friction::new(0.0),
                    ActiveEvents::COLLISION_EVENTS,
                ));
            });
        }
    }
}

use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::TILE_SIZE;

use super::components::Wall;

// TODO: Need to use transform instead of grid_coords for placing the colliders.
pub fn spawn_wall_colliders(
    mut commands: Commands,
    walls: Query<(&GridCoords, Entity, &Transform), Added<Wall>>,
) {
    for (grid_coords, entity, transform) in walls {
        println!(
            "Grid coords: {:?} entity: {:?} transform: {:?}",
            grid_coords, entity, transform
        );
    }
    return;
    let mut all_x_coords_of_y: HashMap<i32, Vec<i32>> = HashMap::new();

    for (grid_coords, entity, transform) in walls {
        let current_x = grid_coords.x;
        let current_y = grid_coords.y;

        all_x_coords_of_y
            .entry(current_y)
            .or_insert_with(Vec::new)
            .push(current_x);
    }

    for (y, all_x) in all_x_coords_of_y {
        println!("\n");
        println!("Current y coordinate: {}", &y);
        println!("x coordinates of current y coordinate: {:?}", &all_x);

        if all_x.is_empty() {
            continue;
        }

        let start_from_collider_x = &all_x[0];
        let mut end_from_collider_x = &all_x[0];

        for (index, current_x) in all_x.iter().enumerate() {
            let next_item = if index == all_x.len() - 1 {
                println!(
                    "No next_item, index: {} all_x.len(): {}",
                    index,
                    all_x.len()
                );
                None
            } else {
                Some(all_x[index + 1])
            };

            end_from_collider_x = current_x;
            if let Some(next_item) = next_item {
                if next_item.abs_diff(*current_x) > 1 {
                    println!(
                        "next_item difference to current_x is bigger than one, should mean GAP. current_x: {} next_item: {}",
                        current_x, next_item
                    );
                    // means our collider should end here
                    // TODO: Also need to continue spawning next
                    // Probably smart to split them up previously, have nested arrays
                    break;
                }
            }
        }

        let middle = (start_from_collider_x + end_from_collider_x) as f32 / 2.0;
        println!(
            "Calculated middle: {} with start_from_collider_x: {} and end_from_collider_x: {}",
            middle, start_from_collider_x, end_from_collider_x
        );

        let cuboid_half_x = all_x.len() as f32 * TILE_SIZE as f32 / 2.0;
        let cuboid_half_y = (TILE_SIZE / 2) as f32;

        let world_x = middle * TILE_SIZE as f32;

        println!("world_x is: {}", world_x);
        println!("Spawning collider at middle: {}", middle);
        println!("Cuboid half_x: {} half_y: {}", cuboid_half_x, cuboid_half_y);
        commands.spawn((
            Transform {
                translation: Vec3::new(world_x, y as f32, 0.0),
                ..Default::default()
            },
            Collider::cuboid(cuboid_half_x, cuboid_half_y),
        ));
        println!("\n");
    }
}

use std::collections::HashMap;

use bevy_ecs_ldtk::GridCoords;

// TODO: Give proper name
// We could even further optimize this, by combining rows together, but its fine for now
pub fn preprocess_grid_coords(wall_grid_coords: Vec<&GridCoords>) -> HashMap<i32, Vec<Vec<i32>>> {
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

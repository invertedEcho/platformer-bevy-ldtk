use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::TILE_SIZE;

use super::components::Wall;

// TODO: This spawns a collider for each wall tile, which is bad for performance. We should spawn
// colliders as large as possible, e.g. check where corner are from walls and span the collider cuboid
pub fn spawn_wall_colliders(
    mut commands: Commands,
    walls: Query<(&GridCoords, Entity), Added<Wall>>,
) {
    let half_tile_size = (TILE_SIZE / 2) as f32;
    for (grid_coords, entity) in walls {
        commands
            .entity(entity)
            .insert(Collider::cuboid(half_tile_size, half_tile_size));
    }
}

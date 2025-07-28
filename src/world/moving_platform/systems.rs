use std::ops::Neg;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_grid_coords_to_translation;
use bevy_rapier2d::prelude::*;

use super::components::{MoveDirection, MovingPlatform};
use super::{
    MOVING_PLATFORM_POINTS_IDENTIFIER, MOVING_PLATFORM_SPEED, MOVING_PLATFORM_TILE_HEIGHT,
    PLATFORM_SINGLE_MIDDLE_SPRITE_PATH,
};

pub fn process_moving_platforms(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    moving_platforms_query: Query<Entity, Added<MovingPlatform>>,
) {
    for moving_platform in moving_platforms_query {
        commands.entity(moving_platform).insert((
            // TODO: im very confused, the width in ldtk is set to 64, so should be 32, but 32
            // is way too big
            Collider::cuboid(8.0, (MOVING_PLATFORM_TILE_HEIGHT / 2) as f32),
            RigidBody::KinematicVelocityBased,
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: Vec2::splat(0.0),
                angvel: 0.0,
            },
            MoveDirection::default(),
            Sprite {
                image: asset_server.load(PLATFORM_SINGLE_MIDDLE_SPRITE_PATH),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    // moving platform is 64 px wide, but image is only 16 px wide.
                    // so, sprite is a quarter of what we want, so 0.25
                    stretch_value: 0.25,
                },
                ..default()
            },
        ));
    }
}

pub fn move_moving_platforms(
    moving_platforms_query: Query<
        (
            &EntityInstance,
            &Transform,
            &mut Velocity,
            &mut MoveDirection,
        ),
        With<MovingPlatform>,
    >,
    layer_query: Query<&LayerMetadata>,
) {
    // TODO: Kinda duplicated from patrol slimes
    for (entity_instance, moving_platform_transform, mut velocity, mut move_direction) in
        moving_platforms_query
    {
        let points: Vec<&IVec2> = entity_instance
            .iter_points_field(MOVING_PLATFORM_POINTS_IDENTIFIER)
            .unwrap()
            .collect();

        if points.len() > 2 {
            error!("More than two points is not yet implemented.");
            continue;
        }

        let entity_layer = layer_query
            .iter()
            .find(|layer| layer.identifier == "Entities")
            .expect("Can find layer with identifier Entities");

        let current_point_ldtk_grid_coord = points[move_direction.index];
        let target_point_translation = ldtk_grid_coords_to_translation(
            *current_point_ldtk_grid_coord,
            entity_layer.c_hei,
            IVec2::splat(16),
        );

        if move_direction.upwards {
            if moving_platform_transform.translation.y < target_point_translation.y {
                velocity.linvel.y = MOVING_PLATFORM_SPEED;
            } else {
                velocity.linvel.y = 0.0;
                move_direction.index = 1;
                move_direction.upwards = false;
            }
        } else {
            if moving_platform_transform.translation.y > target_point_translation.y {
                velocity.linvel.y = MOVING_PLATFORM_SPEED.neg();
            } else {
                velocity.linvel.y = 0.0;
                move_direction.index = 0;
                move_direction.upwards = true;
            }
        }
    }
}

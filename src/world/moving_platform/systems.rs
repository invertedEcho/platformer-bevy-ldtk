use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_grid_coords_to_translation;
use bevy_rapier2d::prelude::*;
use std::ops::Neg;

use crate::HALF_TILE_SIZE;
use crate::player::components::Player;

use super::components::{Direction, MovingPlatform, MovingPlatformInfo};
use super::{
    MOVING_PLATFORM_DIRECTION_IDENTIFIER, MOVING_PLATFORM_POINTS_IDENTIFIER, MOVING_PLATFORM_SPEED,
    MOVING_PLATFORM_TILE_HEIGHT, MOVING_PLATFORM_TILE_WIDTH, PLATFORM_SINGLE_MIDDLE_SPRITE_PATH,
};

pub fn process_moving_platforms(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    moving_platforms_query: Query<(Entity, &EntityInstance), Added<MovingPlatform>>,
) {
    for (entity, entity_instance) in moving_platforms_query {
        let direction_str = entity_instance
            .get_enum_field(MOVING_PLATFORM_DIRECTION_IDENTIFIER)
            .expect("ldtk moving platform direction field correctly typed");

        let direction = get_direction_from_ldtk_enum_value_string(direction_str)
            .expect("Can get direction from direction_str");
        let points = entity_instance
            .iter_points_field(MOVING_PLATFORM_POINTS_IDENTIFIER)
            .expect("ldtk moving platform points field correctly typed")
            .map(|point| *point)
            .collect();

        commands.entity(entity).insert((
            // TODO: im very confused, the width in ldtk is set to 64, so should be 32, but 32
            // is way too big
            Collider::cuboid(8.0, (MOVING_PLATFORM_TILE_HEIGHT / 2) as f32),
            RigidBody::KinematicVelocityBased,
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: Vec2::splat(0.0),
                angvel: 0.0,
            },
            MovingPlatformInfo {
                direction,
                points_index: 0,
                points,
            },
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

pub fn move_moving_platform(
    moving_platform_query: Query<
        (&Transform, &mut Velocity, &mut MovingPlatformInfo),
        (With<MovingPlatform>, Without<Player>),
    >,
    layer_query: Query<&LayerMetadata>,
    mut player_query: Query<(&Player, &mut Velocity), (With<Player>, Without<MovingPlatform>)>,
) {
    let Ok(player) = player_query.single_mut() else {
        return;
    };
    let (player, mut player_velocity) = player;
    for (transform, mut velocity, mut moving_platform_info) in moving_platform_query {
        let points = &moving_platform_info.points;

        if points.len() > 2 {
            error!("More than two points is not yet implemented.");
            continue;
        }

        let entity_layer = layer_query
            .iter()
            .find(|layer| layer.identifier == "Entities")
            .expect("Can find layer with identifier Entities");

        let current_point_ldtk_grid_coord = points[moving_platform_info.points_index];
        let target_point_translation = ldtk_grid_coords_to_translation(
            current_point_ldtk_grid_coord,
            entity_layer.c_hei,
            IVec2::splat(16),
        );

        // Note that a moving platform is 64px wide. The target points are positioned to either
        // very left of platform or very right, so we have to add/substract half of width to get
        // very left or very right of platform, as translation is in the middle per default.
        // and also substract HALF_TILE_SIZE as target point is in center of a coordinate, not
        // at very left or very right of tile.
        let mut current_translation_x_with_offset = transform.translation.x;

        if moving_platform_info.direction == Direction::HorizontalForward {
            current_translation_x_with_offset += MOVING_PLATFORM_TILE_WIDTH / 2.0 - HALF_TILE_SIZE;
        } else if moving_platform_info.direction == Direction::HorizontalBackwards {
            current_translation_x_with_offset -= MOVING_PLATFORM_TILE_WIDTH / 2.0 - HALF_TILE_SIZE;
        }

        match moving_platform_info.direction {
            Direction::VerticalUpwards => {
                if transform.translation.y < target_point_translation.y {
                    velocity.linvel.y = MOVING_PLATFORM_SPEED;
                } else {
                    velocity.linvel.y = 0.0;
                    moving_platform_info.points_index = 1;
                    moving_platform_info.direction = Direction::VerticalDownwards;
                }
            }
            Direction::VerticalDownwards => {
                if transform.translation.y > target_point_translation.y {
                    velocity.linvel.y = MOVING_PLATFORM_SPEED.neg();
                } else {
                    velocity.linvel.y = 0.0;
                    moving_platform_info.points_index = 0;
                    moving_platform_info.direction = Direction::VerticalUpwards;
                }
            }
            Direction::HorizontalForward => {
                if current_translation_x_with_offset < target_point_translation.x {
                    velocity.linvel.x = MOVING_PLATFORM_SPEED;
                    if player.on_horizontal_moving_platform {
                        player_velocity.linvel.x = MOVING_PLATFORM_SPEED;
                    }
                } else {
                    velocity.linvel.x = 0.0;
                    moving_platform_info.points_index = 1;
                    moving_platform_info.direction = Direction::HorizontalBackwards;
                }
            }
            Direction::HorizontalBackwards => {
                if current_translation_x_with_offset > target_point_translation.x {
                    velocity.linvel.x = MOVING_PLATFORM_SPEED.neg();
                    if player.on_horizontal_moving_platform {
                        player_velocity.linvel.x = MOVING_PLATFORM_SPEED.neg();
                    }
                } else {
                    velocity.linvel.x = 0.0;
                    moving_platform_info.points_index = 0;
                    moving_platform_info.direction = Direction::HorizontalForward;
                }
            }
        }
    }
}

fn get_direction_from_ldtk_enum_value_string(input: &String) -> Result<Direction, &str> {
    if input == "Horizontal" {
        return Ok(Direction::HorizontalForward);
    } else if input == "Vertical" {
        return Ok(Direction::VerticalUpwards);
    }
    return Err("No such direction.");
}

pub fn player_collides_with_moving_platform(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    moving_platform_query: Query<(Entity, &MovingPlatformInfo), With<MovingPlatform>>,
) {
    for collision_event in collision_event_reader.read() {
        // TODO: Code is exactly same except true if started or false if stopped. Surely I can do
        // better
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _flags) => {
                let Some(moving_platform) = moving_platform_query
                    .iter()
                    .find(|(e, _)| first_entity == e || second_entity == e)
                else {
                    continue;
                };

                let Some(mut player) = player_query
                    .iter_mut()
                    .find(|(e, _)| first_entity == e || second_entity == e)
                else {
                    continue;
                };
                let moving_platform_direction = &moving_platform.1.direction;
                if *moving_platform_direction == Direction::HorizontalForward
                    || *moving_platform_direction == Direction::HorizontalBackwards
                {
                    player.1.on_horizontal_moving_platform = true;
                }
            }
            CollisionEvent::Stopped(first_entity, second_entity, _flags) => {
                let Some(moving_platform) = moving_platform_query
                    .iter()
                    .find(|(e, _)| first_entity == e || second_entity == e)
                else {
                    continue;
                };

                let Some(mut player) = player_query
                    .iter_mut()
                    .find(|(e, _)| first_entity == e || second_entity == e)
                else {
                    continue;
                };

                let moving_platform_direction = &moving_platform.1.direction;
                if *moving_platform_direction == Direction::HorizontalForward
                    || *moving_platform_direction == Direction::HorizontalBackwards
                {
                    player.1.on_horizontal_moving_platform = false;
                }
            }
        }
    }
}

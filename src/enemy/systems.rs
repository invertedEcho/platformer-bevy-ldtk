use std::ops::Neg;

use crate::{
    TILE_SIZE,
    common::components::{AnimationTimer, TextureAtlasIndices},
    player::heart::resources::PlayerHeartResource,
};

use super::{
    ENEMY_SPEED,
    components::{Patrol, Slime},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};
use bevy_rapier2d::prelude::*;

const SLIME_TILE_SIZE_X: u32 = 16;
const SLIME_TILE_SIZE_Y: u32 = 20;

const SLIME_SPRITE_TILESET: &str = "enemies/slime/slime_walk_anim_strip_15_cropped.png";
const SLIME_TEXTURE_ATLAS_INDICES: TextureAtlasIndices = TextureAtlasIndices { first: 0, last: 14 };

// this array indicates how much we need to move y translation of slime collider in each animation
// tick, as the animated sprite bobbs, e.g. jumps up and down so we also need to move the collider.
// we cannot just move the collider along side as the collider is inserted in the entity coming
// from ldtk, which is 16x16. the animated sprite however is 16x24, as we need the space for
// bobbing.
// const SLIME_COLLIDER_OFFSET_Y_TO_ADD: [f32; 15] = [
//     0., 0., 0., 0., 0., 1., 3., 2., 0., -3., -1., -2., 0., 0., 0.,
// ];

// 4 as tile size is 16, but animated tileset size is 16x24, and first tile starts at bottom of
// tileset, e.g. (24-16) / 2
// const SLIME_TILESET_Y_OFFSET: u32 = 4;

pub fn spawn_slimes(
    mut commands: Commands,
    slime_query: Query<Entity, Added<Slime>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SLIME_SPRITE_TILESET);
    let layout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: SLIME_TILE_SIZE_X,
            y: SLIME_TILE_SIZE_Y,
        },
        15,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let half_slime_size_tile_x = (SLIME_TILE_SIZE_X / 2) as f32;
    let half_slime_size_tile_y = (SLIME_TILE_SIZE_Y / 2) as f32;

    for entity in slime_query {
        commands.entity(entity).insert((
            Collider::cuboid(half_slime_size_tile_x, half_slime_size_tile_y),
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: Vec2::splat(0.0),
                angvel: 0.0,
            },
            LockedAxes::ROTATION_LOCKED,
            RigidBody::KinematicVelocityBased,
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: SLIME_TEXTURE_ATLAS_INDICES.first,
                }),
                flip_x: true,
                ..default()
            },
            SLIME_TEXTURE_ATLAS_INDICES,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn detect_slime_collision_with_player(
    mut collision_event_reader: EventReader<CollisionEvent>,
    slime_query: Query<Entity, With<Slime>>,
    player_query: Query<Entity, With<Slime>>,
    mut player_heart_resource: ResMut<PlayerHeartResource>,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _) = *collision_event else {
            continue;
        };

        let is_collision_entities_slime = slime_query
            .iter()
            .any(|slime| slime == first_entity || slime == second_entity);
        if !is_collision_entities_slime {
            continue;
        }

        let is_collision_entities_player = player_query
            .iter()
            .any(|player| player == first_entity || player == second_entity);
        if !is_collision_entities_player {
            continue;
        }

        if player_heart_resource.count == 0 {
            eprintln!(
                "detect_slime_collision_with_player triggered even though player_heart_resource.count is already 0. Ignoring..."
            );
            continue;
        }

        player_heart_resource.count -= 1;
    }
}

pub fn patrol_slimes(
    slimes_query: Query<
        (
            &EntityInstance,
            &Transform,
            &mut Velocity,
            &mut Patrol,
            &mut Sprite,
        ),
        With<Slime>,
    >,
) {
    for (entity_instance, transform, mut velocity, mut patrol, mut sprite) in slimes_query {
        let patrol_points: Vec<&IVec2> = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correctly typed")
            .collect();

        if patrol_points.len() > 2 {
            eprintln!("More than two patrol points is not yet implemented.");
            continue;
        }

        let current_patrol_point = patrol_points[patrol.index];

        let translated_to_world_coordinate = grid_coords_to_translation(
            GridCoords {
                x: current_patrol_point.x,
                y: current_patrol_point.y,
            },
            IVec2::splat(TILE_SIZE as i32),
        );

        if patrol.forward {
            sprite.flip_x = true;
            if transform.translation.x < translated_to_world_coordinate.x {
                velocity.linvel.x = ENEMY_SPEED;
            } else {
                velocity.linvel.x = 0.0;
                patrol.index = 1;
                patrol.forward = false;
            }
        } else {
            sprite.flip_x = false;
            if transform.translation.x > translated_to_world_coordinate.x {
                velocity.linvel.x = ENEMY_SPEED.neg();
            } else {
                velocity.linvel.x = 0.0;
                patrol.index = 0;
                patrol.forward = true;
            }
        }
    }
}

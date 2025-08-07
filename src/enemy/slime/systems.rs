use std::ops::Neg;

use crate::{
    TILE_SIZE,
    common::components::{AnimationTimer, TextureAtlasIndices},
    enemy::components::Enemy,
    player::components::{Player, PlayerState},
};

use super::{
    SLIME_SPEED,
    components::{Patrol, Slime},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};
use bevy_rapier2d::prelude::*;

const SLIME_TILE_SIZE_X: u32 = 16;
const SLIME_TILE_SIZE_Y: u32 = 20;

const SLIME_SPRITE_TILESET: &str = "enemies/slime/slime_walk_anim_strip_15_cropped.png";
const SLIME_TEXTURE_ATLAS_INDICES: TextureAtlasIndices = TextureAtlasIndices { first: 0, last: 14 };

// maybe reintroduce this, it was for having the collider to be exactly around the animated slime
// for each animation tick:
//
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

pub fn setup_slimes(
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
            AnimationTimer::default(),
            Enemy,
        ));
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
    player_query: Query<&Player>,
) {
    for (entity_instance, transform, mut velocity, mut patrol, mut sprite) in slimes_query {
        for player in player_query {
            if player.state == PlayerState::Dead {
                velocity.linvel.x = 0.0;
                return;
            }
        }
        let patrol_points: Vec<&IVec2> = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correctly typed")
            .collect();

        if patrol_points.len() > 2 {
            error!("More than two patrol points is not yet implemented.");
            continue;
        }

        let current_patrol_point = patrol_points[patrol.points_index];

        // TODO: Need to use ldtk grid coords, as patrol point is ldtk grid coord, not grid coord
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
                velocity.linvel.x = SLIME_SPEED;
            } else {
                velocity.linvel.x = 0.0;
                patrol.points_index = 1;
                patrol.forward = false;
            }
        } else {
            sprite.flip_x = false;
            if transform.translation.x > translated_to_world_coordinate.x {
                velocity.linvel.x = SLIME_SPEED.neg();
            } else {
                velocity.linvel.x = 0.0;
                patrol.points_index = 0;
                patrol.forward = true;
            }
        }
    }
}

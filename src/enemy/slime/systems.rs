use std::ops::Neg;

use crate::{
    HALF_TILE_SIZE, TILE_SIZE,
    common::components::{AnimationTimer, TextureAtlasIndices},
    enemy::components::Enemy,
    player::components::{Player, PlayerState},
};

use super::{
    SLIME_SPEED,
    components::{Patrol, Slime, SlimeCollider},
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
const SLIME_COLLIDER_OFFSET_Y_TO_ADD: [f32; 15] = [
    0., 0., 0., 0., 0., 1., 3., 2., 0., -3., -1., -2., 0., 0., 0.,
];

pub fn setup_slimes(
    mut commands: Commands,
    slime_query: Query<(Entity, &Transform), Added<Slime>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_query: Query<(Entity, &LevelIid)>,
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

    for (entity, transform) in slime_query {
        commands.entity(entity).insert((
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
            RigidBody::KinematicVelocityBased,
            Velocity {
                linvel: Vec2::splat(0.0),
                angvel: 0.0,
            },
        ));

        let level_entity = level_query.single().expect("Can get current level");

        // spawning the slime colliders as level child so they get despawned when the bevy_ecs_ldtk
        // Respawn component gets inserted
        commands.entity(level_entity.0).with_children(|parent| {
            parent.spawn((
                SlimeCollider {
                    slime_entity: entity,
                },
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y - 2.0,
                    transform.translation.z,
                ),
                Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
                ActiveEvents::COLLISION_EVENTS,
                Velocity {
                    linvel: Vec2::splat(0.0),
                    angvel: 0.0,
                },
                LockedAxes::ROTATION_LOCKED,
                RigidBody::KinematicVelocityBased,
                Enemy,
            ));
        });
    }
}

pub fn animate_and_move_collider_slime(
    time: Res<Time>,
    mut slime_query: Query<(&TextureAtlasIndices, &mut AnimationTimer, &mut Sprite), With<Slime>>,
    slime_collider_query: Query<(&SlimeCollider, &mut Transform)>,
    player_query: Query<&Player>,
) {
    // TODO: All these checks if player state is dead and not running the system should just be in
    // .add_systems(run_if) condition, but the "player.state" is not actually a state, so i need to
    // find a different way to do that
    for player in player_query {
        if player.state == PlayerState::Dead {
            return;
        }
    }

    for (slime_collider, mut slime_collider_transform) in slime_collider_query {
        if let Ok(slime) = slime_query.get_mut(slime_collider.slime_entity) {
            let (indices, mut timer, mut sprite) = slime;
            timer.tick(time.delta());

            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    let new_index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                    let offset = SLIME_COLLIDER_OFFSET_Y_TO_ADD[new_index];
                    slime_collider_transform.translation.y += offset;
                    atlas.index = new_index;
                }
            }
        }
    }
}

pub fn patrol_slimes(
    slime_collider_query: Query<(&SlimeCollider, &mut Velocity, &Transform), Without<Slime>>,
    mut slimes_query: Query<
        (&EntityInstance, &mut Patrol, &mut Sprite, &mut Velocity),
        (With<Slime>, Without<SlimeCollider>),
    >,
    player_query: Query<&Player>,
) {
    for (slime_collider, mut velocity_slime_collider, transform_of_slime_collider) in
        slime_collider_query
    {
        if let Ok(slime) = slimes_query.get_mut(slime_collider.slime_entity) {
            let (entity_instance, mut patrol, mut sprite, mut velocity_slime) = slime;

            if let Ok(player) = player_query.single() {
                if player.state == PlayerState::Dead {
                    velocity_slime_collider.linvel.x = 0.0;
                    velocity_slime.linvel.x = 0.0;
                    info!("set velocity of both to 0 and returning.");
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
                if transform_of_slime_collider.translation.x < translated_to_world_coordinate.x {
                    velocity_slime.linvel.x = SLIME_SPEED;
                    velocity_slime_collider.linvel.x = SLIME_SPEED;
                } else {
                    velocity_slime.linvel.x = 0.0;
                    velocity_slime_collider.linvel.x = 0.0;
                    patrol.points_index = 1;
                    patrol.forward = false;
                }
            } else {
                sprite.flip_x = false;
                if transform_of_slime_collider.translation.x > translated_to_world_coordinate.x {
                    velocity_slime_collider.linvel.x = SLIME_SPEED.neg();
                    velocity_slime.linvel.x = SLIME_SPEED.neg();
                } else {
                    velocity_slime.linvel.x = 0.0;
                    velocity_slime_collider.linvel.x = 0.0;
                    patrol.points_index = 0;
                    patrol.forward = true;
                }
            }
        }
    }
}

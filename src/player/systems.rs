use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{TILE_SIZE, jumper::components::Jumper};

use super::components::{AnimationIndices, AnimationTimer, Player};

const PLAYER_SPEED: f32 = 100.0;

pub fn process_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<(Entity, &Transform), Added<Player>>,
) {
    let half_tile_size = (TILE_SIZE / 2) as f32;
    for (entity, transform) in new_players {
        let texture = asset_server.load("herochar/herochar_run_anim_strip_6.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 5 };

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, 3.0),
                scale: transform.scale,
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            RigidBody::Dynamic,
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Collider::cuboid(half_tile_size, half_tile_size),
            LockedAxes::ROTATION_LOCKED,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            ActiveEvents::CONTACT_FORCE_EVENTS,
            Jumper { is_jumping: false },
        ));
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Velocity, &mut Jumper), With<Player>>,
) {
    for (mut velocity, mut jumper) in player.iter_mut() {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        if input.pressed(KeyCode::KeyS) && !jumper.is_jumping {
            velocity.linvel.y = -1.0 * PLAYER_SPEED;
        }
        if input.just_pressed(KeyCode::Space) && !jumper.is_jumping {
            velocity.linvel.y = 220.0;
            jumper.is_jumping = true;
        }
    }
}

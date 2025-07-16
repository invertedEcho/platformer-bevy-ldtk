use crate::{
    common::components::{AnimationIndices, AnimationTimer},
    enemy::components::SlimeSprite,
};

use super::components::Slime;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

const SLIME_TILE_SIZE_X: u32 = 16;
const SLIME_TILE_SIZE_Y: u32 = 24;

const SLIME_SPRITE_TILESET: &str = "enemies/slime/slime_walk_anim_strip_15.png";
const SLIME_SPRITE_ANIMATION_INDICES: AnimationIndices = AnimationIndices { first: 0, last: 14 };

const SLIME_Y_OFFSET_BOBBING_FOR_EACH_TILE: [f32; 1] = [4.];

// this array indicates how much we need to move y translation of slime collider in each animation
// tick
const MOVE_SLIME_Y: [i32; 16] = [0, -2, -1, 4, 1, 1, 2, 3, 1, -2, -2, -2, -3, -2, 1, 2];

pub fn spawn_slimes(
    mut commands: Commands,
    slime_query: Query<(Entity, &Transform), Added<Slime>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SLIME_SPRITE_TILESET);
    let layout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: SLIME_TILE_SIZE_X,
            y: SLIME_TILE_SIZE_Y,
        },
        16,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let half_slime_size_tile_x = (SLIME_TILE_SIZE_X / 2) as f32;
    // let half_slime_size_tile_y = (SLIME_TILE_SIZE_Y / 2) as f32;

    for (slime_entity, transform) in slime_query {
        println!("transform: {:?}", transform);
        commands.entity(slime_entity).insert((Collider::cuboid(
            half_slime_size_tile_x,
            half_slime_size_tile_x,
        ),));

        let translation = transform.translation;
        let y_offset = SLIME_Y_OFFSET_BOBBING_FOR_EACH_TILE[0];

        commands.spawn((
            Transform::from_xyz(translation.x, translation.y + y_offset, 3.0),
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: SLIME_SPRITE_ANIMATION_INDICES.first,
                },
            ),
            SLIME_SPRITE_ANIMATION_INDICES,
            SlimeSprite,
            // AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn animate_and_move_slime(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut Sprite,
            &mut Transform,
        ),
        With<Slime>,
    >,
    mut slime_sprite_query: Query<&mut Transform, With<SlimeSprite>>,
) {
    for (indices, mut timer, mut sprite, mut transform) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let move_by_y = MOVE_SLIME_Y[atlas.index];
                if move_by_y < 0 {
                    transform.translation.y -= move_by_y.abs() as f32;
                } else {
                    transform.translation.y += move_by_y.abs() as f32;
                }

                atlas.index = if atlas.index == indices.last {
                    transform.translation.y = 56.0;
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

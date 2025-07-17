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

// this array indicates how much we need to move y translation of slime collider in each animation
// tick
const MOVE_SLIME_Y: [f32; 16] = [
    0., -2., -1., 4., 1., 1., 2., 3., 1., -2., -2., -2., -3., -2., 1., 2.,
];

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

    for (entity, transform) in slime_query {
        println!("slime_transform (with collider): {:?}", transform);

        commands.entity(entity).insert((Collider::cuboid(
            half_slime_size_tile_x,
            half_slime_size_tile_x,
        ),));

        let translation = transform.translation;

        commands.spawn((
            Transform::from_xyz(translation.x, translation.y + 4.0, 3.0),
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: SLIME_SPRITE_ANIMATION_INDICES.first,
                },
            ),
            SLIME_SPRITE_ANIMATION_INDICES,
            SlimeSprite,
            AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
        ));
    }
}

pub fn animate_and_move_slime(
    time: Res<Time>,
    mut slime_collider_query: Query<&mut Transform, With<Slime>>,
    mut slime_sprite_query: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut Sprite),
        With<SlimeSprite>,
    >,
) {
    for (index, (indices, mut timer, mut sprite)) in slime_sprite_query.iter_mut().enumerate() {
        // TODO: This is ugly, this wont get the exact corresponding Slime Collider Transform for the current SlimeSprite.
        // Find a better way for this
        if let Some(mut slime_collider) = slime_collider_query.iter_mut().nth(index) {
            timer.tick(time.delta());

            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    let offset = MOVE_SLIME_Y[atlas.index];
                    slime_collider.translation.y += offset;
                    atlas.index = if atlas.index == indices.last {
                        // set back to origin
                        slime_collider.translation.y = 56.;
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

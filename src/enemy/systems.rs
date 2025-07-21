use crate::{
    common::components::{AnimationTimer, TextureAtlasIndices},
    enemy::components::SlimeSprite,
    player::heart::resources::PlayerHeartResource,
};

use super::components::Slime;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SLIME_TILE_SIZE_X: u32 = 16;
const SLIME_TILE_SIZE_Y: u32 = 24;

const SLIME_SPRITE_TILESET: &str = "enemies/slime/slime_walk_anim_strip_15.png";
const SLIME_TEXTURE_ATLAS_INDICES: TextureAtlasIndices = TextureAtlasIndices { first: 0, last: 14 };

// this array indicates how much we need to move y translation of slime collider in each animation
// tick, as the animated sprite bobbs, e.g. jumps up and down so we also need to move the collider.
// we cannot just move the collider along side as the collider is inserted in the entity coming
// from ldtk, which is 16x16. the animated sprite however is 16x24, as we need the space for
// bobbing.
const SLIME_COLLIDER_OFFSET_Y_TO_ADD: [f32; 15] = [
    0., 0., 0., 0., 0., 1., 3., 2., 0., -3., -1., -2., 0., 0., 0.,
];

// 4 as tile size is 16, but animated tileset size is 16x24, and first tile starts at bottom of
// tileset, e.g. (24-16) / 2
const INITIAL_SLIME_SPRITE_Y_OFFSET: f32 = 4.0;

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

    let half_slime_size_tile = (SLIME_TILE_SIZE_X / 2) as f32;

    for (entity, transform) in slime_query {
        commands.entity(entity).insert((
            Collider::cuboid(half_slime_size_tile, half_slime_size_tile),
            ActiveEvents::COLLISION_EVENTS,
        ));

        let translation = transform.translation;

        let translation_y = translation.y + INITIAL_SLIME_SPRITE_Y_OFFSET;

        commands.spawn((
            Transform::from_xyz(translation.x, translation_y, 3.0),
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: SLIME_TEXTURE_ATLAS_INDICES.first,
                },
            ),
            SLIME_TEXTURE_ATLAS_INDICES,
            SlimeSprite,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn animate_and_move_slime(
    time: Res<Time>,
    mut slime_collider_query: Query<&mut Transform, With<Slime>>,
    mut slime_sprite_query: Query<
        (&TextureAtlasIndices, &mut AnimationTimer, &mut Sprite),
        With<SlimeSprite>,
    >,
) {
    for (index, (indices, mut timer, mut sprite)) in slime_sprite_query.iter_mut().enumerate() {
        // TODO: This is ugly, this wont get the exact corresponding Slime Collider Transform for the current SlimeSprite.
        // Find a better way for this
        let Some(mut slime_collider) = slime_collider_query.iter_mut().nth(index) else {
            return;
        };
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
                let slime_collider_y_offset = SLIME_COLLIDER_OFFSET_Y_TO_ADD[atlas.index];
                slime_collider.translation.y += slime_collider_y_offset;
            }
        }
    }
}

pub fn detect_slime_collision_with_player(
    mut event_reader: EventReader<CollisionEvent>,
    slime_query: Query<Entity, With<Slime>>,
    player_query: Query<Entity, With<Slime>>,
    mut player_heart_resource: ResMut<PlayerHeartResource>,
) {
    for event in event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _collision_event_flags) = event
        else {
            continue;
        };

        let is_slime = slime_query
            .iter()
            .any(|slime| slime == *first_entity || slime == *second_entity);
        if !is_slime {
            continue;
        }

        let is_player = player_query
            .iter()
            .any(|player| player == *first_entity || player == *second_entity);

        if !is_player {
            continue;
        }

        if player_heart_resource.count == 0 {
            eprintln!(
                "detect_slime_collision_with_player triggered even though player_heart_resource.count is already 0. Ignoring..."
            );
            continue;
        }

        println!("Player collided with a slime!");
        player_heart_resource.count -= 1;
        println!("Player heart count: {}", player_heart_resource.count);
    }
}

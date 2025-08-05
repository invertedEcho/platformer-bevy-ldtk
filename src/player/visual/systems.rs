use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;

use crate::common::NORMAL_ANIMATION_TIMER_DURATION;
use crate::player::components::{Player, PlayerDeadAnimationTimer, PlayerDirection, PlayerState};
use crate::player::visual::{
    PLAYER_DEATH_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT,
    PLAYER_DEATH_ANIM_TILESET_PATH, PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES,
    PLAYER_JUMP_UP_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES,
};

use crate::TILE_SIZE;
use crate::common::components::TextureAtlasIndices;

use super::{
    PLAYER_IDLE_ANIM_TILESET_PATH, PLAYER_JUMP_UP_ANIM_STRIP_PATH, PLAYER_RUN_ANIM_TILESET_PATH,
};

pub fn handle_player_change_visual(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    player_query: Query<(Entity, &Player, &mut Sprite, &mut TextureAtlasIndices), Changed<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, player, mut sprite, mut texture_atlas_indices) in player_query {
        info!("Player has changed: {:?}", player);
        match player.state {
            PlayerState::Idle => {
                info!("setting idle sprite");
                let texture = asset_server.load(PLAYER_IDLE_ANIM_TILESET_PATH);
                let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                *sprite = Sprite {
                    image: texture,
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    }),
                    flip_x: player.direction == PlayerDirection::Backwards,
                    ..default()
                };
                *texture_atlas_indices = PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES;
            }
            PlayerState::Run => {
                info!("setting run sprite");
                let texture = asset_server.load(PLAYER_RUN_ANIM_TILESET_PATH);
                let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                *sprite = Sprite {
                    image: texture,
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    }),
                    flip_x: player.direction == PlayerDirection::Backwards,
                    ..default()
                };
                *texture_atlas_indices = PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES;
            }
            PlayerState::Jump => {
                info!("setting jump up sprite");
                let texture = asset_server.load(PLAYER_JUMP_UP_ANIM_STRIP_PATH);
                let texture_atlas_layout =
                    TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 1, None, None);
                let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);
                *sprite = Sprite {
                    image: texture,
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout_handle,
                        index: 0,
                    }),
                    flip_x: player.direction == PlayerDirection::Backwards,
                    ..default()
                };
                *texture_atlas_indices = PLAYER_JUMP_UP_ANIM_TEXTURE_ATLAS_INDICES;
            }
            PlayerState::Dead => {
                let texture_atlas_layout = TextureAtlasLayout::from_grid(
                    UVec2::splat(TILE_SIZE as u32),
                    PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT,
                    1,
                    None,
                    None,
                );
                let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);
                let texture_atlas = TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index: 0,
                };

                commands.entity(entity).insert((
                    Sprite::from_atlas_image(
                        asset_server.load(PLAYER_DEATH_ANIM_TILESET_PATH),
                        texture_atlas,
                    ),
                    PLAYER_DEATH_ANIM_TEXTURE_ATLAS_INDICES,
                    PlayerDeadAnimationTimer(Timer::from_seconds(
                        NORMAL_ANIMATION_TIMER_DURATION
                            * PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT as f32,
                        TimerMode::Once,
                    )),
                    // fixed rigidbody so player collider doesnt move because of colliding with enemy
                    RigidBody::Fixed,
                ));
            }
        }
    }
}

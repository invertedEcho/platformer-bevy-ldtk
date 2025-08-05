use bevy::prelude::*;

use crate::player::components::{Player, PlayerDirection};
use crate::player::visual::{
    PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_JUMP_UP_ANIM_TEXTURE_ATLAS_INDICES,
    PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES,
};

use crate::TILE_SIZE;
use crate::common::components::TextureAtlasIndices;

use super::{
    PLAYER_IDLE_ANIM_TILESET_PATH, PLAYER_JUMP_UP_ANIM_STRIP_PATH, PLAYER_RUN_ANIM_TILESET_PATH,
};

pub fn set_forward_player_run_sprite(
    asset_server: Res<AssetServer>,
    player_query: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut sprite, mut animation_indices) in player_query {
        let texture = asset_server.load(PLAYER_RUN_ANIM_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES;
    }
}

pub fn set_forward_idle_player_sprite(
    asset_server: Res<AssetServer>,
    player_query: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut sprite, mut animation_indices) in player_query {
        info!("setting forward idle player sprite");
        let texture = asset_server.load(PLAYER_IDLE_ANIM_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES;
    }
}

pub fn set_backwards_player_run_sprite(
    asset_server: Res<AssetServer>,
    player_query: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut sprite, mut animation_indices) in player_query {
        let texture = asset_server.load(PLAYER_RUN_ANIM_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES;
        sprite.flip_x = true;
    }
}

pub fn set_backwards_idle_sprite(
    asset_server: Res<AssetServer>,
    player_query: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut sprite, mut texture_atlas_indices) in player_query {
        let texture = asset_server.load(PLAYER_IDLE_ANIM_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *texture_atlas_indices = PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES;
        sprite.flip_x = true;
    }
}

pub fn set_jump_sprite(
    asset_server: Res<AssetServer>,
    player_query: Query<(&Player, &mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (player, mut sprite, mut texture_atlas_indices) in player_query {
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
}

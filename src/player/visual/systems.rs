use bevy::prelude::*;

use crate::player::components::Player;
use crate::player::visual::{
    PLAYER_BACKWARDS_IDLE_TILESET_PATH, PLAYER_BACKWARDS_RUN_SPRITE_ANIMATION_INDICES,
    PLAYER_BACKWARDS_RUN_SPRITE_TILESET, PLAYER_BACKWARDS_SPRITE_ANIMATION_INDICES,
    PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES, PLAYER_FORWARD_RUN_SPRITE_ANIMATION_INDICES,
};

use crate::common::components::TextureAtlasIndices;

use super::{PLAYER_FORWARD_IDLE_SPRITE_TILESET, PLAYER_FORWARD_RUN_SPRITE_TILESET};

// TODO: Instead of all this, just use flip_x: true to get backwards run/idle

pub fn set_forward_player_run_sprite(
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_FORWARD_RUN_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_FORWARD_RUN_SPRITE_ANIMATION_INDICES;
    }
}

pub fn set_forward_idle_player_sprite(
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_FORWARD_IDLE_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES;
    }
}

pub fn set_backwards_player_run_sprite(
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_BACKWARDS_RUN_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_BACKWARDS_RUN_SPRITE_ANIMATION_INDICES;
    }
}

pub fn set_backwards_idle_sprite(
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_BACKWARDS_IDLE_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_BACKWARDS_SPRITE_ANIMATION_INDICES;
    }
}

use bevy::prelude::*;

use crate::player::components::Player;
use crate::player::visual::{
    PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES,
};

use crate::common::components::TextureAtlasIndices;

use super::{PLAYER_IDLE_ANIM_TILESET_PATH, PLAYER_RUN_ANIM_TILESET_PATH};

pub fn set_forward_player_run_sprite(
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
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
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
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
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
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
    player: Query<(&mut Sprite, &mut TextureAtlasIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut texture_atlas_indices) in player {
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

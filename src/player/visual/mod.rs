use crate::common::components::TextureAtlasIndices;

pub mod systems;

const PLAYER_FORWARD_RUN_SPRITE_TILESET: &str = "herochar/herochar_run_forward_anim_strip_6.png";
const PLAYER_FORWARD_RUN_SPRITE_ANIMATION_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 5 };

const PLAYER_BACKWARDS_RUN_SPRITE_TILESET: &str =
    "herochar/herochar_run_backwards_anim_strip_6.png";
const PLAYER_BACKWARDS_RUN_SPRITE_ANIMATION_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 5 };

pub const PLAYER_FORWARD_IDLE_SPRITE_TILESET: &str = "herochar/herochar_forward_idle_anim_strip_4.png";
pub const PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 3 };

const PLAYER_BACKWARDS_IDLE_SPRITE_TILESET: &str =
    "herochar/herochar_backwards_idle_anim_strip_4.png";
const PLAYER_BACKWARDS_IDLE_SPRITE_ANIMATION_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 3 };

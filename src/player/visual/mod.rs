use crate::common::components::TextureAtlasIndices;

pub mod systems;

const PLAYER_RUN_ANIM_TILESET_PATH: &str = "herochar/herochar_run_anim_strip_6.png";
const PLAYER_RUN_ANIM_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 5 };

pub const PLAYER_IDLE_ANIM_TILESET_PATH: &str = "herochar/herochar_idle_anim_strip_4.png";
pub const PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 3 };

pub const PLAYER_DEATH_ANIM_TILESET_PATH: &str = "herochar/herochar_death_anim_strip_8.png";
pub const PLAYER_DEATH_ANIM_TEXTURE_ATLAS_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 7 };

pub const PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT: u32 = 8;

pub const PLAYER_JUMP_ANIM_STRIP_PATH: &str = "herochar/herochar_jump_anim_strip_3.png";

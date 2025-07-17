use bevy::prelude::*;

#[derive(Component)]
/// To be used with tilesets of animated sprites
pub struct TextureAtlasIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

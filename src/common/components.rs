use bevy::prelude::*;

use super::NORMAL_ANIMATION_TIMER_DURATION;

#[derive(Component)]
pub struct TextureAtlasIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(
            NORMAL_ANIMATION_TIMER_DURATION,
            TimerMode::Repeating,
        ))
    }
}

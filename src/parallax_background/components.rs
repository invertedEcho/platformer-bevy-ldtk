use bevy::prelude::*;

#[derive(Component)]
pub struct ParallaxBackground {
    // TODO: Use this to check if background off screen and move to the very right of all
    // backgrounds
    // pub width: f32,
    pub speed: f32,
    pub initial_x_position: f32,
}

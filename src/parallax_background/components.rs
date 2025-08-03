use bevy::prelude::*;

#[derive(Component)]
pub struct ParallaxBackground {
    pub width: f32,
    pub speed: f32,
    pub initial_x_position: f32,
}

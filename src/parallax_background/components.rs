use bevy::prelude::*;

#[derive(Component)]
pub struct ParallaxBackground {
    pub speed: f32,
    pub initial_x_position: f32,
}

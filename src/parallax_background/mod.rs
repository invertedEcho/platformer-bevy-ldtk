use bevy::prelude::*;
use systems::{
    handle_parallax_background_relative_to_camera, handle_window_resize, setup_parallax_background,
};

mod components;
mod systems;

// maybe better to just use our component struct to have named fields
// tuple for each layer:
// 0. path
// 1. speed
// 2. image width
const PARALLAX_BACKGROUND_LAYERS: [(&str, f32, f32); 6] = [
    ("sky.png", 0.1, 320.0),
    ("far-clouds.png", 0.2, 128.0),
    ("near-clouds.png", 0.25, 144.0),
    ("far-mountains.png", 0.35, 160.0),
    ("mountains.png", 0.55, 320.0),
    ("trees.png", 0.6, 240.0),
];

/// All parallax backgrounds have the same height
const PARALLAX_BACKGROUND_HEIGHT: f32 = 240.;

pub struct ParallaxBackgroundPlugin;

impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_parallax_background)
            .add_systems(
                Update,
                (
                    handle_parallax_background_relative_to_camera,
                    handle_window_resize,
                ),
            );
    }
}

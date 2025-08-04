use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::CAMERA_SCALE;

use super::{
    PARALLAX_BACKGROUND_HEIGHT, PARALLAX_BACKGROUND_LAYERS, components::ParallaxBackground,
};

pub const Z_INDEX: [f32; 6] = [1.0, 1.1, 1.2, 1.3, 1.4, 1.5];

pub fn setup_parallax_background(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().expect("Can get primary window");

    let scaled_window_height = window.height() * CAMERA_SCALE;
    let scale = scaled_window_height / PARALLAX_BACKGROUND_HEIGHT;

    for (index, (path, speed, width)) in PARALLAX_BACKGROUND_LAYERS.iter().enumerate() {
        let image_handle: Handle<Image> =
            asset_server.load("parallax_background/".to_string() + path);

        let mut x_position = -(window.width() / 2.0);
        let scaled_window_width = window.width() * scale;

        while x_position < scaled_window_width {
            commands.spawn((
                ParallaxBackground {
                    width: *width,
                    speed: *speed,
                    initial_x_position: x_position,
                },
                Transform {
                    translation: Vec3 {
                        x: x_position,
                        y: 0.0,
                        z: Z_INDEX[index],
                    },
                    scale: Vec3::splat(scale),
                    ..default()
                },
                Sprite {
                    image: image_handle.clone(),
                    ..default()
                },
            ));
            x_position += width;
        }
    }
}

pub fn handle_parallax_background_relative_to_camera(
    background_query: Query<
        (&ParallaxBackground, &mut Transform),
        (With<ParallaxBackground>, Without<Camera>),
    >,
    camera_query: Query<&Transform, (With<Camera>, Without<ParallaxBackground>)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for (parallax_background, mut transform) in background_query {
        let parallax_x = parallax_background.initial_x_position
            + camera_transform.translation.x * (1.0 - parallax_background.speed);

        transform.translation.x = parallax_x;
        transform.translation.y = camera_transform.translation.y;
    }
}

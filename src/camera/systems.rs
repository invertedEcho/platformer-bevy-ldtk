use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::components::Player;

const CAMERA_SCALE: f32 = 0.4;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: -CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    window_dimensions: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let mut camera_transform = camera_query
        .single_mut()
        .expect("Exactly one camera should exist");

    let window_dimension = window_dimensions.single().unwrap();

    // follow player, but (these comments are bad, i only understand them because i know what it
    // does, but reading them makes no sense, i just dont know how to express this)
    // - left edge of camera should not go below level width
    // - bottom edge of camera should not go below level height
    // TODO: right edge of camera should not go above level width

    let half_window_width = window_dimension.width() / 2.0;
    let new_camera_translation_x =
        (half_window_width * CAMERA_SCALE).max(player_transform.translation.x);

    let half_window_height = window_dimension.height() / 2.0;
    let new_camera_translation_y =
        (half_window_height * CAMERA_SCALE).max(player_transform.translation.y);

    println!("camera_translation_x: {}", camera_transform.translation.x);
    println!("player_translation_x: {}", player_transform.translation.x);
    println!("half_window_width: {}", half_window_width);
    println!("window_width: {}", window_dimension.width());

    camera_transform.translation.x = new_camera_translation_x;
    camera_transform.translation.y = new_camera_translation_y;
}

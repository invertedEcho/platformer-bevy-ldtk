use crate::camera::CAMERA_SCALE;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

use crate::player::components::Player;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: -CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn camera_follow_player_with_level_clamping(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    window_dimensions: Query<&Window, With<PrimaryWindow>>,
    level_query: Query<&LevelIid, (Without<Projection>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let mut camera_transform = camera_query
        .single_mut()
        .expect("Exactly one camera should exist");

    let window_dimensions = window_dimensions.single().unwrap();

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single().unwrap())
        .unwrap();

    let Some(current_level_width) = level_query.iter().find_map(|level_iid| {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .unwrap();

        level_selection
            .is_match(&LevelIndices::default(), level)
            .then_some(level.px_wid)
    }) else {
        error!("Failed to find level, camera_follow_player may be broken.");
        return;
    };

    // follow player, but (these comments are bad, i only understand them because i know what it
    // does, but reading them makes no sense, i just dont know how to express this)
    // - left edge of camera should not go below level width
    // - bottom edge of camera should not go below level height

    let half_window_width = window_dimensions.width() / 2.0;
    let new_camera_translation_x =
        (half_window_width * CAMERA_SCALE).max(player_transform.translation.x);

    let half_window_height = window_dimensions.height() / 2.0;
    let new_camera_translation_y =
        (half_window_height * CAMERA_SCALE).max(player_transform.translation.y);

    // right edge of camera should not go further than level width
    if new_camera_translation_x + half_window_width * CAMERA_SCALE < current_level_width as f32 {
        camera_transform.translation.x = new_camera_translation_x;
    }

    camera_transform.translation.y = new_camera_translation_y;
}

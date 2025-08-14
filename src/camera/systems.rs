use crate::camera::CAMERA_SCALE;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

use crate::player::components::Player;

const SMOOTHING_FACTOR: f32 = 0.3;

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
    let window_height = window_dimensions.height();
    let window_width = window_dimensions.width();

    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single().unwrap())
        .unwrap();

    let Some(current_level) = level_query.iter().find_map(|level_iid| {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .unwrap();

        // TODO: why levelindices? we dont use indices
        level_selection
            .is_match(&LevelIndices::default(), level)
            .then_some(level)
    }) else {
        error!("Failed to find level, camera_follow_player may be broken.");
        return;
    };

    let current_level_width = current_level.px_wid as f32;
    let current_level_height = current_level.px_hei as f32;

    let half_window_width = window_width / 2.0;

    // left edge of camera should not go beyond level width
    let new_camera_translation_x =
        (half_window_width * CAMERA_SCALE).max(player_transform.translation.x);

    // right edge of camera should not go beyond level width
    if new_camera_translation_x + half_window_width * CAMERA_SCALE < current_level_width {
        camera_transform.translation.x +=
            (new_camera_translation_x - camera_transform.translation.x) * SMOOTHING_FACTOR;
    }

    // bottom of camera should not go below level height
    let half_window_height = window_height / 2.0;
    let new_camera_translation_y =
        (half_window_height * CAMERA_SCALE).max(player_transform.translation.y);

    let top_of_player = player_transform.translation.y + half_window_height * CAMERA_SCALE;

    // top of camera should not go above level height
    if top_of_player > current_level_height {
        return;
    }

    camera_transform.translation.y +=
        (new_camera_translation_y - camera_transform.translation.y) * SMOOTHING_FACTOR;
}

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

use crate::player::components::Player;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: -0.4,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    level_query: Query<&LevelIid, (Without<Camera2d>, Without<Player>)>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    window_dimensions: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let mut camera_transform = camera_query
        .single_mut()
        .expect("Exactly one camera should exist");

    let window_dimension = window_dimensions.single().unwrap();

    // keep camera bottom at window bottom
    // TODO: use min so if player goes up, camera also goes up, but never below window
    let half_window_height = window_dimension.height() / 2.0;
    camera_transform.translation.y = half_window_height * 0.4;

    // keep left side of camera minimum at window left
    let half_window_width = window_dimension.width() / 2.0;
    camera_transform.translation.x = half_window_width * 0.4;

    // *camera_transform = *player_transform;
    // camera_transform.translation.y = 0.0;
    println!("Camera transform: {:?}", camera_transform);

    // for level_id in level_query {
    //     let ldtk_project = ldtk_project_assets
    //         .get(ldtk_projects.single().unwrap())
    //         .unwrap();
    //
    //     let level = ldtk_project
    //         .get_raw_level_by_iid(&level_id.to_string())
    //         .unwrap();
    //
    //     println!("Level INFO: {:?}", level);
    // }
}

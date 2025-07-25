use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::coins::resources::CoinResource;

pub fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
    input: Res<ButtonInput<KeyCode>>,
    mut coin_resource: ResMut<CoinResource>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        commands
            .entity(
                ldtk_projects
                    .single()
                    .expect("Exactly one ldtk project exists"),
            )
            .insert(Respawn);
        coin_resource.count = 0;
    }
}

pub fn skip_to_next_level(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if !cfg!(debug_assertions) {
        return;
    }
    if keyboard_input.just_released(KeyCode::ArrowRight) {
        *level_selection = LevelSelection::Identifier("Level_1".to_string());
    }
}

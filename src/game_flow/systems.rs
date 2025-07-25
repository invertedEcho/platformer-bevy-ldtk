use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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

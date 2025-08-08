use bevy::prelude::*;

use crate::state::GameState;

use super::components::PauseMenuRoot;

pub fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            PauseMenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node { ..default() }, Button))
                .with_children(|parent| {
                    parent.spawn(Text::new("Resume"));
                });
            parent
                .spawn((Node { ..default() }, Button))
                .with_children(|parent| {
                    parent.spawn(Text::new("Quit"));
                });
        });
}

pub fn handle_escape_pause(
    button_input: Res<ButtonInput<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if button_input.just_pressed(KeyCode::Escape) {
        if *current_game_state == GameState::InGame {
            next_game_state.set(GameState::Paused);
        } else if *current_game_state == GameState::Paused {
            next_game_state.set(GameState::InGame);
        }
    }
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenuRoot>>,
) {
    for pause_menu in pause_menu_query {
        commands.entity(pause_menu).despawn();
    }
}

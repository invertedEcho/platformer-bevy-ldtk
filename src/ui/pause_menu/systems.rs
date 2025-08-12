use bevy::prelude::*;

use crate::{
    state::GameState,
    ui::common::components::{CommonButtonType, CommonUiButton},
};

use super::components::{PauseMenuButton, PauseMenuButtonType, PauseMenuRoot};

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
                .spawn((
                    Node { ..default() },
                    Button,
                    PauseMenuButton {
                        pause_menu_button_type: PauseMenuButtonType::Resume,
                    },
                ))
                .with_child(Text::new("Resume"));
            parent
                .spawn((
                    Node { ..default() },
                    Button,
                    CommonUiButton {
                        button_type: CommonButtonType::BackToMainMenu,
                    },
                ))
                .with_child(Text::new("Back to Main Menu"));
            parent
                .spawn((Node { ..default() }, Button))
                .with_child(Text::new("Quit"));
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

pub fn handle_pause_menu_button_pressed(
    mut next_game_state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, &PauseMenuButton), Changed<Interaction>>,
) {
    for (interaction, pause_menu_button) in query {
        let Interaction::Pressed = interaction else {
            continue;
        };
        match pause_menu_button.pause_menu_button_type {
            PauseMenuButtonType::Resume => next_game_state.set(GameState::InGame),
        }
    }
}

use bevy::prelude::*;

use crate::{
    game_save::utils::reset_game_save,
    state::GameState,
    ui::common::components::{CommonButtonType, CommonUiButton},
};

use super::components::{SettingsButton, SettingsButtonType, SettingsRoot};

pub fn spawn_settings_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            SettingsRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node { ..default() },
                    Button,
                    SettingsButton {
                        settings_button_type: SettingsButtonType::ResetGameSave,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Reset game save"));
                });
            parent
                .spawn((
                    Node { ..default() },
                    Button,
                    CommonUiButton {
                        button_type: CommonButtonType::BackToMainMenu,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Back"));
                });
        });
}

pub fn despawn_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsRoot>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_button_press(
    interaction_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, settings_button) in interaction_query {
        let Interaction::Pressed = interaction else {
            continue;
        };
        match settings_button.settings_button_type {
            SettingsButtonType::ResetGameSave => {
                reset_game_save();
                next_game_state.set(GameState::InGame);
            }
        }
    }
}

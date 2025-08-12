use bevy::prelude::*;

use crate::{
    state::GameState,
    ui::common::components::{CommonButtonType, CommonUiButton},
};

use super::components::{MainMenuButton, MainMenuButtonType, MainMenuRoot};

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("A platformer"));
            // empty gap
            parent.spawn(Node {
                height: Val::Percent(10.0),
                ..default()
            });
            // play button
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    MainMenuButton {
                        main_menu_button_type: MainMenuButtonType::Play,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Play"));
                });
            // settings button
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    MainMenuButton {
                        main_menu_button_type: MainMenuButtonType::Settings,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Settings"));
                });
            // quit button
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    CommonUiButton {
                        button_type: CommonButtonType::Quit,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Quit"));
                });
        });
}

pub fn handle_interaction_pressed(
    interaction_query: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, main_menu_button) in interaction_query {
        match interaction {
            Interaction::Pressed => match main_menu_button.main_menu_button_type {
                MainMenuButtonType::Play => next_game_state.set(GameState::InGame),
                MainMenuButtonType::Settings => next_game_state.set(GameState::Settings),
            },
            _ => {}
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenuRoot>>,
) {
    for main_menu in main_menu_query {
        commands.entity(main_menu).despawn();
    }
}

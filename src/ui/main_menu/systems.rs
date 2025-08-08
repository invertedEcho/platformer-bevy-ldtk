use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;

use crate::{LEVEL_IIDS, state::GameState};

use super::components::MainMenuRoot;

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
            // play button
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Play"));
                });
            // settings button
            // parent
            //     .spawn((
            //         Node {
            //             justify_content: JustifyContent::Center,
            //             align_items: AlignItems::Center,
            //             ..default()
            //         },
            //         Button,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(Text::new("Settings"));
            //     });
            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Quit"));
                });
        });
}

// TODO: this handles both interactions from main menu and pause menu
pub fn handle_interaction(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<(&Text, &mut TextColor)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interaction, children) in interaction_query {
        if let Ok(result) = text_query.get_mut(children[0]) {
            let (text, mut text_color) = result;
            match *interaction {
                Interaction::Hovered => **text_color = Color::hsl(39.0, 1.0, 0.5),
                Interaction::None => **text_color = Color::WHITE,
                Interaction::Pressed => {
                    // TODO: this is horrible, dont match by text
                    if **text == "Play" {
                        commands.insert_resource(LevelSelection::iid(LEVEL_IIDS[0]));
                        next_game_state.set(GameState::InGame);
                    } else if **text == "Quit" {
                        app_exit_writer.write(AppExit::Success);
                    } else if **text == "Resume" {
                        next_game_state.set(GameState::InGame);
                    }
                }
            }
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

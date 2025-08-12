use bevy::prelude::*;

use crate::{state::GameState, ui::common::components::CommonButtonType};

use super::components::CommonUiButton;

pub fn handle_button_interaction_hover(
    interaction_query: Query<(&Interaction, &Children), (With<Button>, Changed<Interaction>)>,
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, children) in interaction_query {
        if let Ok(mut text_color) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Hovered => **text_color = Color::hsl(39.0, 1.0, 0.5),
                Interaction::None => **text_color = Color::WHITE,
                _ => {}
            }
        }
    }
}

pub fn handle_common_button_press(
    mut app_exit_writer: EventWriter<AppExit>,
    interaction_query: Query<(&Interaction, &CommonUiButton), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, common_ui_button) in interaction_query {
        let Interaction::Pressed = interaction else {
            continue;
        };
        match common_ui_button.button_type {
            CommonButtonType::BackToMainMenu => {
                next_game_state.set(GameState::MainMenu);
            }
            CommonButtonType::Quit => {
                app_exit_writer.write(AppExit::Success);
            }
        }
    }
}

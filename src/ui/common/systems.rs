use bevy::prelude::*;

use crate::{state::GameState, ui::common::components::CommonButtonType};

use super::components::CommonUiButton;

pub fn handle_common_button_press(
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
        }
    }
}

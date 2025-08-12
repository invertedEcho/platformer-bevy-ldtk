use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    EMPTY_LEVEL_IID, coins::resources::CoinResource, game_save::utils::get_or_create_game_save,
};

pub fn handle_enter_main_menu_state(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Iid(LevelIid::new(EMPTY_LEVEL_IID)));
}

pub fn handle_enter_in_game_state(mut commands: Commands, mut coin_resource: ResMut<CoinResource>) {
    let current_game_save = get_or_create_game_save();
    coin_resource.count = current_game_save.player_coins;
    commands.insert_resource(LevelSelection::iid(current_game_save.level_iid));
}

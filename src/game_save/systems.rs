use crate::coins::resources::CoinResource;
use bevy::prelude::*;

use super::utils::get_or_create_game_save;

pub fn load_game_save(mut coin_resource: ResMut<CoinResource>) {
    let current_game_save = get_or_create_game_save();
    coin_resource.count = current_game_save.player_coins;
}

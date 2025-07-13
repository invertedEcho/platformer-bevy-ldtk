use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::{Coin, CoinBundle};
use resources::CoinResource;
use systems::{coin_collision_detection, process_coins};

use crate::common::systems::animate_generic_sprite;

mod components;
pub mod resources;
mod systems;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoinResource>()
            .register_ldtk_entity_for_layer::<CoinBundle>("Coins", "Coins")
            .add_systems(
                Update,
                (
                    process_coins,
                    animate_generic_sprite::<Coin>,
                    coin_collision_detection,
                ),
            );
    }
}

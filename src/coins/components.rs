use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Coin;

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
pub struct CoinBundle {
    coin: Coin,
}

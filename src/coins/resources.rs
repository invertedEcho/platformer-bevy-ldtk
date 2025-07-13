use bevy::prelude::*;

#[derive(Resource)]
pub struct CoinResource {
    pub count: i32,
}

impl Default for CoinResource {
    fn default() -> Self {
        CoinResource { count: 0 }
    }
}

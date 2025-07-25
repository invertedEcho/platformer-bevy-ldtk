use bevy::prelude::*;

pub const INITIAL_PLAYER_HEART_COUNT: u32 = 3;

#[derive(Resource)]
pub struct PlayerHeartResource {
    pub count: u32,
}

impl Default for PlayerHeartResource {
    fn default() -> Self {
        PlayerHeartResource {
            count: INITIAL_PLAYER_HEART_COUNT,
        }
    }
}

use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerHeartResource {
    pub count: u32
}

impl Default for PlayerHeartResource {
    fn default() -> Self {
        PlayerHeartResource {
            count: 3
        }
    }
}

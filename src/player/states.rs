use bevy::prelude::*;

#[derive(Default, Hash, Clone, Debug, Eq, PartialEq, States)]
pub enum PlayerState {
    #[default]
    Alive,
    Respawning,
}

use bevy::prelude::*;

// TODO: This can just be idle, run, or jump, because now we have player.direction
// TODO: Also, this should not be a state. State is for global stuff. We should just have property in player
// component and use Changed<Player> to reflect the changes visually, e.g. sprites
#[derive(States, Eq, PartialEq, Clone, Hash, Debug, Default)]
pub enum PlayerMovementType {
    #[default]
    ForwardIdle,
    ForwardRun,
    BackwardsIdle,
    BackwardsRun,
    Jump,
}

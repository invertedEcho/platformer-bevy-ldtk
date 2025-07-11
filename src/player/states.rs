use bevy::prelude::*;

#[derive(States, Eq, PartialEq, Clone, Hash, Debug, Default)]
pub enum PlayerMovementType {
    #[default]
    ForwardIdle,
    ForwardRun,
    BackwardsIdle,
    BackwardsRun,
}

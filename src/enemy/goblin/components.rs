use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Goblin {
    pub state: GoblinState,
}

#[derive(Default, Debug, PartialEq)]
pub enum GoblinState {
    #[default]
    Idle,
    RunBackwards,
    RunForward,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoblinBundle {
    goblin: Goblin,
}

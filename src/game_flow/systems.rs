use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::EMPTY_LEVEL_IID;

pub fn handle_enter_main_menu_state(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Iid(LevelIid::new(EMPTY_LEVEL_IID)));
}

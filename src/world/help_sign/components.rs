use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct HelpSign;

#[derive(Bundle, LdtkEntity, Default)]
pub struct HelpSignBundle {
    help_sign: HelpSign,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Component, Default)]
pub struct KeyboardTile;

#[derive(Bundle, Default, LdtkEntity)]
pub struct KeyboardTileBundle {
    keyboard_tile: KeyboardTile,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

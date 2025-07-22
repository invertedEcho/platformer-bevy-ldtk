use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Slime;

#[derive(LdtkEntity, Default, Bundle)]
pub struct SlimeBundle {
    enemy: Slime,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

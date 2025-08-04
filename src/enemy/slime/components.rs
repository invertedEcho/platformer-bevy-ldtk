use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Slime;

#[derive(LdtkEntity, Default, Bundle)]
pub struct SlimeBundle {
    slime: Slime,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    patrol: Patrol,
}

#[derive(Component)]
pub struct Patrol {
    pub forward: bool,
    pub points_index: usize,
}

impl Default for Patrol {
    fn default() -> Self {
        Patrol {
            forward: true,
            points_index: 0,
        }
    }
}

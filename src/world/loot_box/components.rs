use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, LdtkEntity, Bundle)]
pub struct LootBoxBundle {
    loot_box: LootBox,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Component, Default)]
pub struct LootBox {
    pub is_opened: bool,
}

#[derive(Component)]
pub struct LootBoxOpeningTimer(pub Timer);

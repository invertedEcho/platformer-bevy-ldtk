use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, LdtkEntity, Bundle)]
pub struct LootBoxBundle {
    loot_box: LootBox,
}

#[derive(Component, Default)]
pub struct LootBox {
    is_opened: bool,
}

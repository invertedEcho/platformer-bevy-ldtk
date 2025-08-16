use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::components::AnimationTimer,
    world::loot_box::{
        LOOT_BOX_ANIM_STRIP_PATH, LOOT_BOX_HEIGHT, LOOT_BOX_TEXTURE_ATLAS_INDICES, LOOT_BOX_WIDTH,
        components::LootBox,
    },
};

pub fn process_loot_boxes(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<Entity, Added<LootBox>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: LOOT_BOX_WIDTH as u32,
            y: LOOT_BOX_HEIGHT as u32,
        },
        12,
        1,
        None,
        None,
    );
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);

    for loot_box_entity in query {
        commands.entity(loot_box_entity).insert((
            Sprite {
                image: asset_server.load(LOOT_BOX_ANIM_STRIP_PATH),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout_handle.clone(),
                    index: 0,
                }),
                ..default()
            },
            AnimationTimer::default(),
            LOOT_BOX_TEXTURE_ATLAS_INDICES,
            Collider::cuboid(LOOT_BOX_WIDTH / 2.0, LOOT_BOX_HEIGHT / 2.0),
        ));
    }
}

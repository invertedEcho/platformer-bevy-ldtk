use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::{
        NORMAL_ANIMATION_TIMER_DURATION,
        components::{AnimationTimer, TextureAtlasIndices},
    },
    game_save::utils::{get_or_create_game_save, update_game_save},
    player::components::Player,
    world::loot_box::{
        LOOT_BOX_ANIM_STRIP_PATH, LOOT_BOX_CLOSED_HEIGHT, LOOT_BOX_CLOSED_WIDTH,
        LOOT_BOX_OPEN_HEIGHT, LOOT_BOX_OPEN_SPRITE_PATH, LOOT_BOX_OPEN_WIDTH,
        LOOT_BOX_OPENING_ANIM_STRIP_PATH, LOOT_BOX_OPENING_TEXTURE_ATLAS_INDICES,
        LOOT_BOX_TEXTURE_ATLAS_INDICES, Loot,
        components::{LootBox, LootBoxOpeningTimer},
        utils::ldtk_loot_enum_to_rust_enum,
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
            x: LOOT_BOX_CLOSED_WIDTH as u32,
            y: LOOT_BOX_CLOSED_HEIGHT as u32,
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
            Collider::cuboid(LOOT_BOX_CLOSED_WIDTH / 2.0, LOOT_BOX_CLOSED_HEIGHT / 2.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

// TODO: i think it would be really cool if we dont just open the loot box when we collide but the
// user needs to press a key to actually open it. and then add some text above the loot box to show
// the user how to open, e.g. which key to press
pub fn detect_player_collision_with_loot_box(
    mut collision_event_reader: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    mut loot_box_query: Query<
        (Entity, &EntityInstance, &mut LootBox, &mut Collider),
        With<LootBox>,
    >,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _flags) = *collision_event else {
            continue;
        };

        let Some(collided_loot_box) =
            loot_box_query
                .iter_mut()
                .find(|(loot_box_entity, _, _, _)| {
                    *loot_box_entity == first_entity || *loot_box_entity == second_entity
                })
        else {
            continue;
        };

        if collided_loot_box.2.is_opened {
            continue;
        }

        let is_collided_entities_player = player_query
            .iter()
            .any(|e| e == first_entity || e == second_entity);
        if !is_collided_entities_player {
            continue;
        }

        let (_, entity_instance, mut loot_box, mut collider) = collided_loot_box;

        let loot_ldtk_string = entity_instance
            .get_enum_field("Loot")
            .expect("ldtk field exists");
        let loot_enum = ldtk_loot_enum_to_rust_enum(loot_ldtk_string);

        if loot_enum == Loot::GrapplingHook {
            info!("looted grappling hook!");
            let mut game_save = get_or_create_game_save();
            game_save.grappling_hook_unlocked = true;
            update_game_save(&game_save);
            info!("updated game save with grappling_hook_unlocked = true");
            loot_box.is_opened = true;
            *collider = Collider::cuboid(8.0, 5.0);
        }
    }
}

pub fn handle_loot_box_change_visual(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<(Entity, &LootBox, &mut Sprite), Changed<LootBox>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, changed_loot_box, mut sprite) in query {
        // TODO: Technically its not opened here, but it was collided, and the opening animation
        // should start. Would be better if is_opened actually first happens when opening animation
        // is done
        if changed_loot_box.is_opened {
            info!("Loot Box has changed to is_opened, setting visually opening");
            sprite.image = asset_server.load(LOOT_BOX_OPENING_ANIM_STRIP_PATH);

            let texture_atlas = TextureAtlasLayout::from_grid(
                UVec2 {
                    x: LOOT_BOX_OPEN_WIDTH,
                    y: LOOT_BOX_OPEN_HEIGHT,
                },
                6,
                1,
                None,
                None,
            );
            let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas);
            sprite.texture_atlas = Some(TextureAtlas {
                index: 0,
                layout: texture_atlas_layout_handle,
            });

            commands.entity(entity).insert((
                LOOT_BOX_OPENING_TEXTURE_ATLAS_INDICES,
                LootBoxOpeningTimer(Timer::from_seconds(
                    6.0 * NORMAL_ANIMATION_TIMER_DURATION,
                    TimerMode::Once,
                )),
            ));
        }
    }
}

pub fn handle_loot_box_opening_timer(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &mut Sprite, &mut LootBoxOpeningTimer), With<LootBoxOpeningTimer>>,
) {
    for (entity, mut sprite, mut loot_box_opening_timer) in query {
        loot_box_opening_timer.0.tick(time.delta());
        if loot_box_opening_timer.0.finished() {
            info!("Loot box opening timer finished, setting sprite image to static open");
            sprite.image = asset_server.load(LOOT_BOX_OPEN_SPRITE_PATH);
            info!("Removing AnimationTimer, TextureAtlasIndices and LootBoxOpeningTimer");
            commands.entity(entity).remove::<AnimationTimer>();
            commands.entity(entity).remove::<TextureAtlasIndices>();
            commands.entity(entity).remove::<LootBoxOpeningTimer>();
        }
    }
}

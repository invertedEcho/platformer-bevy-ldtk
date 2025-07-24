use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::components::AnimationTimer,
    player::components::Player,
    world::save_point::{
        SAVE_POINT_SAVING_ANIM_STRIP_PATH, SAVE_POINT_SAVING_TEXTURE_ATLAS_INDICES,
        components::SavingSavePointTimer,
    },
};

use super::{SAVE_POINT_ANIM_STRIP_PATH, SAVE_POINT_TEXTURE_ATLAS_INDICES, components::SavePoint};

pub fn process_save_points(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    save_point_query: Query<Entity, Added<SavePoint>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SAVE_POINT_ANIM_STRIP_PATH);
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 12, y: 20 }, 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let texture_atlas = TextureAtlas {
        index: 0,
        layout: texture_atlas_layout,
    };
    for save_point in save_point_query {
        commands.entity(save_point).insert((
            Sprite::from_atlas_image(texture.clone(), texture_atlas.clone()),
            SAVE_POINT_TEXTURE_ATLAS_INDICES,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Collider::cuboid(6.0, 10.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

pub fn detect_player_collider_with_save_point(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    save_point_query: Query<Entity, With<SavePoint>>,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _) = *collision_event else {
            continue;
        };

        let Some(colliding_save_point) = save_point_query
            .iter()
            .find(|entity| *entity == first_entity || *entity == second_entity)
        else {
            continue;
        };

        let is_collider_entities_player = player_query
            .iter()
            .any(|player| player == first_entity || player == second_entity);
        if !is_collider_entities_player {
            continue;
        }

        let texture = asset_server.load(SAVE_POINT_SAVING_ANIM_STRIP_PATH);
        let texture_atlas_layout =
            TextureAtlasLayout::from_grid(UVec2 { x: 12, y: 20 }, 3, 1, None, None);

        let layout = texture_atlas_layouts.add(texture_atlas_layout);
        let atlas = TextureAtlas { layout, index: 0 };
        let new_sprite = Sprite::from_atlas_image(texture, atlas);

        commands.entity(colliding_save_point).insert((
            new_sprite,
            SAVE_POINT_SAVING_TEXTURE_ATLAS_INDICES,
            SavingSavePointTimer(Timer::from_seconds(1.5, TimerMode::Once)),
        ));
    }
}

// TODO: this function doesnt only tick the timer, but also replaces sprite with normal save point
pub fn tick_saving_save_point_timer(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
    timer_query: Query<(Entity, &mut SavingSavePointTimer)>,
) {
    for (entity, mut saving_save_point_timer) in timer_query {
        saving_save_point_timer.0.tick(time.delta());

        if saving_save_point_timer.0.finished() {
            let texture = asset_server.load(SAVE_POINT_ANIM_STRIP_PATH);
            let layout = TextureAtlasLayout::from_grid(UVec2 { x: 12, y: 20 }, 9, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let texture_atlas = TextureAtlas {
                index: 0,
                layout: texture_atlas_layout,
            };
            commands.entity(entity).remove::<SavingSavePointTimer>();
            commands.entity(entity).insert((
                Sprite::from_atlas_image(texture.clone(), texture_atlas.clone()),
                SAVE_POINT_TEXTURE_ATLAS_INDICES,
            ));
        }
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::components::{AnimationTimer, TextureAtlasIndices},
    player::components::Player,
};

use super::{components::Coin, resources::CoinResource};

const COIN_TILE_SIZE: i32 = 8;
const COIN_SPRITE_TILESET: &str = "miscellaneous sprites/coin_anim_strip_6.png";
const COIN_ANIMATION_INDICES: TextureAtlasIndices = TextureAtlasIndices { first: 0, last: 5 };

pub fn process_coins(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    coin_query: Query<(Entity, &Transform), Added<Coin>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let half_tile_size = (COIN_TILE_SIZE / 2) as f32;

    for (entity, transform) in coin_query {
        let coin_texture = asset_server.load(COIN_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                coin_texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: COIN_ANIMATION_INDICES.first,
                },
            ),
            *transform,
            Collider::cuboid(half_tile_size, half_tile_size),
            ActiveEvents::COLLISION_EVENTS,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            COIN_ANIMATION_INDICES,
        ));
    }
}

pub fn coin_collision_detection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
    coin_query: Query<Entity, With<Coin>>,
    mut coin_resource: ResMut<CoinResource>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _collision_event_flags) => {
                let player_entity = players
                    .single()
                    .expect("Player exists when coin is touched");
                let collision_entities_is_coin = coin_query
                    .iter()
                    .any(|e| e == *first_entity || e == *second_entity);
                if collision_entities_is_coin {
                    if *first_entity == player_entity {
                        commands.entity(*second_entity).despawn();
                    } else if *second_entity == player_entity {
                        commands.entity(*first_entity).despawn();
                    }
                    coin_resource.count += 1;
                }
            }
            _ => {}
        }
    }
}

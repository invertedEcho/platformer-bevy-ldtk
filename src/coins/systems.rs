use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    common::components::{AnimationIndices, AnimationTimer},
    player::components::Player,
};

use super::{components::Coin, resources::CoinResource};

const COIN_TILE_SIZE: i32 = 8;
const COIN_SPRITE_TILESET: &str = "miscellaneous sprites/coin_anim_strip_6.png";
const COIN_ANIMATION_INDICES: AnimationIndices = AnimationIndices { first: 0, last: 5 };

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
            CollisionEvent::Started(entity1, entity2, _collision_event_flags) => {
                let player_entity = players
                    .single()
                    .expect("Player exists when coin is touched");
                let coin_entity = coin_query.iter().find(|e| e == entity1 || e == entity2);
                if let Some(_) = coin_entity {
                    if *entity1 == player_entity {
                        commands.entity(*entity2).despawn();
                    } else if *entity2 == player_entity {
                        commands.entity(*entity1).despawn();
                    }
                    coin_resource.count += 1;
                }
            }
            _ => {}
        }
    }
}

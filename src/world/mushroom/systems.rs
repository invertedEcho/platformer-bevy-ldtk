use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    common::components::{AnimationTimer, TextureAtlasIndices},
    player::{components::Player, movement::PLAYER_JUMP_HIGH},
};

use super::components::Mushroom;

const MUSHROOM_SPRITE_TILESET: &str = "enemies/mushroom/mushroom_crushed_anim_strip_6.png";
const MUSHROOM_SPRITE_ANIMATION_INDICES: TextureAtlasIndices =
    TextureAtlasIndices { first: 0, last: 5 };

pub fn spawn_mushroom_colliders(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mushrooms_query: Query<Entity, Added<Mushroom>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for mushroom in mushrooms_query {
        commands.entity(mushroom).insert((
            Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
            ActiveEvents::COLLISION_EVENTS,
            AnimationTimer::default(),
            Sprite::from_atlas_image(
                asset_server.load(MUSHROOM_SPRITE_TILESET),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: MUSHROOM_SPRITE_ANIMATION_INDICES.first,
                },
            ),
            MUSHROOM_SPRITE_ANIMATION_INDICES,
        ));
    }
}

pub fn mushroom_collision_detection(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mushrooms_query: Query<Entity, With<Mushroom>>,
    mut player_query: Query<(Entity, &mut Velocity), With<Player>>,
) {
    for collision_event in collision_event_reader.read() {
        // TODO: use let else continue
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let collision_entities_is_mushroom = mushrooms_query
                    .iter()
                    .any(|e| e == *first_entity || e == *second_entity);
                if collision_entities_is_mushroom {
                    let (player_entity, mut player_velocity) = player_query
                        .single_mut()
                        .expect("Player exists when colliding with mushroom");
                    if *first_entity == player_entity || *second_entity == player_entity {
                        player_velocity.linvel.y = PLAYER_JUMP_HIGH;
                    }
                }
            }
            _ => {}
        }
    }
}

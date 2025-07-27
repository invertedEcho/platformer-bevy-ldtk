use crate::{
    HALF_TILE_SIZE,
    common::components::{AnimationTimer, TextureAtlasIndices},
    player::components::Player,
};

use super::components::NextLevelOrb;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

const ORB_ANIM_STRIP_PATH: &str = "miscellaneous sprites/orb_anim_strip_6.png";
const ORB_TEXTURE_ATLAS_INDICES: TextureAtlasIndices = TextureAtlasIndices { first: 0, last: 5 };

const NEXT_LEVEL_ORB_FIELD_IDENTIFIER: &str = "Target_Level_IID";

pub fn process_next_level_orbs(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    next_level_orb_query: Query<Entity, Added<NextLevelOrb>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(ORB_ANIM_STRIP_PATH);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for next_level_orb in next_level_orb_query {
        commands.entity(next_level_orb).insert((
            Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                },
            ),
            AnimationTimer::default(),
            ORB_TEXTURE_ATLAS_INDICES,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

pub fn detect_player_next_level_orb_collision(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    next_level_orb_query: Query<(Entity, &EntityInstance), With<NextLevelOrb>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for collision_event in collision_event_reader.read() {
        let CollisionEvent::Started(first_entity, second_entity, _) = *collision_event else {
            continue;
        };
        let Some(colliding_next_level_orb) =
            next_level_orb_query.iter().find(|(next_level_orb, _)| {
                first_entity == *next_level_orb || second_entity == *next_level_orb
            })
        else {
            continue;
        };

        let is_entities_player = player_query
            .iter()
            .any(|player| first_entity == player || second_entity == player);

        if !is_entities_player {
            continue;
        }

        for player in player_query {
            println!("despawning player in next_level logic to ensure smooth sailing...");
            commands.entity(player).despawn();
        }

        let (_, entity_instance) = colliding_next_level_orb;

        let target_level_iid = entity_instance
            .get_string_field(NEXT_LEVEL_ORB_FIELD_IDENTIFIER)
            .expect("Ldtk entity field correctly typed and set");

        *level_selection = LevelSelection::iid(target_level_iid);
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    common::components::AnimationTimer,
};

use super::{components::Player, visual::{PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES, PLAYER_FORWARD_IDLE_SPRITE_TILESET}};

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<(Entity, &mut Transform), Added<Player>>,
) {
    let texture = asset_server.load(PLAYER_FORWARD_IDLE_SPRITE_TILESET);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for (entity, mut transform) in new_players {
        transform.translation.z = 3.0;
        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES.first,
                },
            ),
            PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            RigidBody::Dynamic,
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Collider::cuboid(HALF_TILE_SIZE / 2.0, HALF_TILE_SIZE),
            LockedAxes::ROTATION_LOCKED,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
        ));
    }
}


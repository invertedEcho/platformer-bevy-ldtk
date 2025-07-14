use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    common::components::{AnimationIndices, AnimationTimer},
    jumper::components::Jumper,
    world::platform::components::Platform,
};

use super::{components::Player, states::PlayerMovementType};

const PLAYER_SPEED: f32 = 200.0;

const PLAYER_FORWARD_RUN_SPRITE_TILESET: &str = "herochar/herochar_run_forward_anim_strip_6.png";
const PLAYER_FORWARD_RUN_SPRITE_ANIMATION_INDICES: AnimationIndices =
    AnimationIndices { first: 0, last: 5 };

const PLAYER_BACKWARDS_RUN_SPRITE_TILESET: &str =
    "herochar/herochar_run_backwards_anim_strip_6.png";
const PLAYER_BACKWARDS_RUN_SPRITE_ANIMATION_INDICES: AnimationIndices =
    AnimationIndices { first: 0, last: 5 };

const PLAYER_FORWARD_IDLE_SPRITE_TILESET: &str = "herochar/herochar_forward_idle_anim_strip_4.png";
const PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES: AnimationIndices =
    AnimationIndices { first: 0, last: 3 };

const PLAYER_BACKWARDS_IDLE_SPRITE_TILESET: &str =
    "herochar/herochar_backwards_idle_anim_strip_4.png";
const PLAYER_BACKWARDS_IDLE_SPRITE_ANIMATION_INDICES: AnimationIndices =
    AnimationIndices { first: 0, last: 3 };

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<(Entity, &Transform), Added<Player>>,
) {
    let texture = asset_server.load(PLAYER_FORWARD_IDLE_SPRITE_TILESET);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for (entity, transform) in new_players {
        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES.first,
                },
            ),
            Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, 3.0),
                scale: transform.scale,
                ..default()
            },
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
            Jumper { is_jumping: false },
        ));
    }
}

pub fn set_forward_player_run_sprite(
    player: Query<(&mut Sprite, &mut AnimationIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_FORWARD_RUN_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_FORWARD_RUN_SPRITE_ANIMATION_INDICES;
    }
}

pub fn set_forward_idle_player_sprite(
    player: Query<(&mut Sprite, &mut AnimationIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_FORWARD_IDLE_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES;
    }
}

pub fn set_backwards_player_run_sprite(
    player: Query<(&mut Sprite, &mut AnimationIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_BACKWARDS_RUN_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_BACKWARDS_RUN_SPRITE_ANIMATION_INDICES;
    }
}

pub fn set_backwards_idle_sprite(
    player: Query<(&mut Sprite, &mut AnimationIndices), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, mut animation_indices) in player {
        let texture = asset_server.load(PLAYER_BACKWARDS_IDLE_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        *sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        );
        *animation_indices = PLAYER_BACKWARDS_IDLE_SPRITE_ANIMATION_INDICES;
    }
}

pub fn player_movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Jumper, &Player), With<Player>>,
    current_player_movement_type: Res<State<PlayerMovementType>>,
    mut next_player_movement_type: ResMut<NextState<PlayerMovementType>>,
    platform_query: Query<Entity, With<Platform>>,
) {
    for (mut velocity, mut jumper, player) in player_query.iter_mut() {
        velocity.linvel.x = 0.0;
        let current_player_movement_type = current_player_movement_type.get().clone();
        if current_player_movement_type == PlayerMovementType::BackwardsRun
            || current_player_movement_type == PlayerMovementType::BackwardsIdle
        {
            next_player_movement_type.set(PlayerMovementType::BackwardsIdle);
        } else {
            next_player_movement_type.set(PlayerMovementType::ForwardIdle);
        }
        if input.pressed(KeyCode::KeyD) {
            velocity.linvel.x = 1.0 * PLAYER_SPEED;
            next_player_movement_type.set(PlayerMovementType::ForwardRun);
        }
        if input.pressed(KeyCode::KeyA) {
            velocity.linvel.x = -1.0 * PLAYER_SPEED;
            next_player_movement_type.set(PlayerMovementType::BackwardsRun);
        }
        if input.pressed(KeyCode::KeyS) && !jumper.is_jumping && player.is_on_platform {
            // TODO: This wont work, we need to only insert ColliderDisabled on platforms where
            // user is staying on
            for platform_entity in platform_query {
                commands.entity(platform_entity).insert(ColliderDisabled);
            }
        }
        if input.just_pressed(KeyCode::Space) && !jumper.is_jumping {
            velocity.linvel.y = 220.0;
            jumper.is_jumping = true;
        }
    }
}

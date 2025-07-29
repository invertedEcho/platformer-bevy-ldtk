use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE, TILE_SIZE,
    common::{NORMAL_ANIMATION_TIMER_DURATION, components::AnimationTimer},
    player::{
        components::PlayerDeadAnimationTimer,
        visual::{
            PLAYER_DEATH_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT,
            PLAYER_DEATH_ANIM_TILESET_PATH, components::PlayerDebugLine,
        },
    },
};

use super::{
    components::Player,
    events::PlayerDeadEvent,
    states::PlayerState,
    visual::{PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_IDLE_ANIM_TILESET_PATH},
};

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<(Entity, &mut Transform), Added<Player>>,
) {
    for (entity, mut transform) in new_players {
        info!(
            "Setting up player. This means a new entity was spawned that contains the Player component."
        );
        let texture = asset_server.load(PLAYER_IDLE_ANIM_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        transform.translation.z = 3.0;

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES.first,
                },
            ),
            PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES,
            AnimationTimer::default(),
            RigidBody::Dynamic,
            // TODO: get rid of these magic numbers
            Collider::capsule_y(HALF_TILE_SIZE - 5.0, 6.0),
            LockedAxes::ROTATION_LOCKED,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        ));

        commands.spawn((
            Transform::from_xyz(transform.translation.x, transform.translation.y, 101.0),
            PlayerDebugLine,
            Sprite {
                image: asset_server.load("red_16x2.png"),
                ..default()
            },
        ));
    }
}

pub fn player_debug_line_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<PlayerDebugLine>)>,
    mut player_debug_line_query: Query<&mut Transform, (With<PlayerDebugLine>, Without<Player>)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut player_debug_line) = player_debug_line_query.single_mut() else {
        return;
    };
    player_debug_line.translation = player.translation;
    // should be at feet of player
    player_debug_line.translation.y -= HALF_TILE_SIZE + 1.0;
}

pub fn handle_player_dead_event(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut player_dead_event_reader: EventReader<PlayerDeadEvent>,
    player_query: Query<Entity, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for _ in player_dead_event_reader.read() {
        info!(
            "Received player dead event. Setting PlayerState::Respawning and setting player sprite to death animation"
        );
        for entity in player_query {
            let texture_atlas_layout = TextureAtlasLayout::from_grid(
                UVec2::splat(TILE_SIZE as u32),
                PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT,
                1,
                None,
                None,
            );
            let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);
            let texture_atlas = TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: 0,
            };

            next_player_state.set(PlayerState::Respawning);
            commands.entity(entity).insert((
                Sprite::from_atlas_image(
                    asset_server.load(PLAYER_DEATH_ANIM_TILESET_PATH),
                    texture_atlas,
                ),
                PLAYER_DEATH_ANIM_TEXTURE_ATLAS_INDICES,
                PlayerDeadAnimationTimer(Timer::from_seconds(
                    NORMAL_ANIMATION_TIMER_DURATION * PLAYER_DEATH_ANIM_TILESET_COLUMN_COUNT as f32,
                    TimerMode::Once,
                )),
                // fixed rigidbody so player collider doesnt move because of touching with slime
                RigidBody::Fixed,
            ));
        }
    }
}

pub fn tick_player_dead_animation_timer(
    time: Res<Time>,
    player_query: Query<&mut PlayerDeadAnimationTimer, With<Player>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for mut player_dead_animation_timer in player_query {
        player_dead_animation_timer.tick(time.delta());
        if player_dead_animation_timer.finished() {
            next_player_state.set(PlayerState::Alive);
        }
    }
}

pub fn handle_player_state_enter_alive(
    mut commands: Commands,
    player_query: Query<(Entity, &Player), With<Player>>,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
) {
    for (entity, player) in player_query {
        if let Some(current_save_point) = player.current_save_point {
            info!("Found current save point. Despawning player and spawning new one.");
            // Note that we only need to spawn player with correct transform, as our
            // `process_player` system takes care of everything else.
            commands.entity(entity).despawn();
            commands.spawn((
                Player {
                    current_save_point: Some(current_save_point),
                    ..default()
                },
                Transform::from_xyz(
                    current_save_point.x + TILE_SIZE,
                    current_save_point.y,
                    current_save_point.z,
                ),
            ));
        } else {
            info!("No save_point, respawning level.");
            commands
                .entity(
                    ldtk_projects
                        .single()
                        .expect("Exactly one ldtk project exists"),
                )
                .insert(Respawn);
        }
    }
}

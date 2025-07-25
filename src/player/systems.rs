use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE, TILE_SIZE,
    common::components::AnimationTimer,
    player::{
        components::PlayerDeadAnimationTimer,
        visual::{PLAYER_DEATH_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_DEATH_ANIM_TILESET_PATH},
    },
};

use super::{
    components::Player,
    events::PlayerDeadEvent,
    states::PlayerState,
    visual::{PLAYER_FORWARD_IDLE_SPRITE_ANIMATION_INDICES, PLAYER_FORWARD_IDLE_SPRITE_TILESET},
};

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<(Entity, &mut Transform), Added<Player>>,
) {
    // i think the issue here is that when we reuse transform
    for (entity, mut transform) in new_players {
        println!(
            "Setting up player. This means a new entity was spawned that contains the Player component."
        );
        let texture = asset_server.load(PLAYER_FORWARD_IDLE_SPRITE_TILESET);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        transform.translation.z = 3.0;
        println!("Current transform of player: {}", transform.translation);
        println!(
            "Player should be spawned at: {}",
            grid_coords_to_translation(GridCoords { x: 3, y: 40 }, IVec2 { x: 16, y: 16 })
        );
        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
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

pub fn handle_player_dead_event(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut player_dead_event_reader: EventReader<PlayerDeadEvent>,
    player_query: Query<Entity, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for _ in player_dead_event_reader.read() {
        println!(
            "Received player dead event. Setting PlayerState::Respawning and setting player sprite to death animation"
        );
        for entity in player_query {
            let texture_atlas_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 8, 1, None, None);
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
                // TODO: Replace with constants. 0.1 is our normal animation timer seconds, and 8 is
                // count of columns of player animated death tileset
                PlayerDeadAnimationTimer(Timer::from_seconds(0.1 * 8.0, TimerMode::Once)),
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
        println!("PlayerState entered alive.");

        if let Some(current_save_point) = player.current_save_point {
            println!("Found current save point. Despawning player and spawning new one.");
            // TODO: Unfortunately, changing translation of player to save_point doesnt work.
            // Im assuming the physics engine sets back the position,
            // because the wiki says its discouraged to manually set the position of a
            // dynamic rigidbody.

            // So we must despawn the player and spawn new player...
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
            println!("No save_point, respawning level.");
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

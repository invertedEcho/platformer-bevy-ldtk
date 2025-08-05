use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE, TILE_SIZE, common::components::AnimationTimer,
    player::components::PlayerDeadAnimationTimer,
};

use super::{
    components::Player,
    visual::{PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_IDLE_ANIM_TILESET_PATH},
};

const PLAYER_CAPSULE_RADIUS: f32 = 5.0;

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
        transform.translation.z = 4.0;

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
            // radius increases half_height
            // we need to substract RADIUS + 1 from half_height to get correct size
            Collider::capsule_y(
                HALF_TILE_SIZE - PLAYER_CAPSULE_RADIUS,
                PLAYER_CAPSULE_RADIUS,
            ),
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
    }
}

pub fn tick_player_dead_animation_timer(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(Entity, &mut Player, &mut PlayerDeadAnimationTimer), With<Player>>,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
) {
    for (entity, player, mut player_dead_animation_timer) in player_query {
        player_dead_animation_timer.tick(time.delta());
        if player_dead_animation_timer.finished() {
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
}

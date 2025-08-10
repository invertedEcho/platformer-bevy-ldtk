use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE, TILE_SIZE, common::components::AnimationTimer,
    ground_detection::components::GroundDetection, player::components::PlayerDeadAnimationTimer,
};

use super::{
    components::{Player, PlayerState},
    visual::{PLAYER_IDLE_ANIM_TEXTURE_ATLAS_INDICES, PLAYER_IDLE_ANIM_TILESET_PATH},
};

const PLAYER_CAPSULE_RADIUS: f32 = 5.0;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<Entity, Added<Player>>,
) {
    for entity in new_players {
        info!(
            "Setting up player. This means a new entity was spawned that contains the Player component."
        );
        let texture = asset_server.load(PLAYER_IDLE_ANIM_TILESET_PATH);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

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
            // we need to substract RADIUS from half_height to get correct size
            Collider::capsule_y(
                HALF_TILE_SIZE - PLAYER_CAPSULE_RADIUS,
                PLAYER_CAPSULE_RADIUS,
            ),
            LockedAxes::ROTATION_LOCKED,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            Friction::new(0.0),
            GroundDetection { on_ground: true },
        ));
    }
}

pub fn tick_player_dead_animation_timer(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(Entity, &mut Player, &mut PlayerDeadAnimationTimer), With<Player>>,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
) {
    for (entity, mut player, mut player_dead_animation_timer) in player_query {
        player_dead_animation_timer.tick(time.delta());
        if player_dead_animation_timer.finished() {
            player.state = PlayerState::Idle;
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
                        1.0,
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

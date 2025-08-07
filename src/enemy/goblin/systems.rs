use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE,
    common::components::AnimationTimer,
    enemy::{
        ENEMY_EXCLAMATION_MARK_ANIM_STRIP_PATH, ENEMY_EXCLAMATION_MARK_ANIM_TEXTURE_ATLAS_INDICES,
        components::{Enemy, EnemyTriggered},
        events::EnemyTriggeredEvent,
    },
    player::components::Player,
};

use super::{
    GOBLIN_IDLE_ANIM_STRIP_PATH, GOBLIN_IDLE_ANIM_TEXTURE_ATLAS_INDICES,
    GOBLIN_RUN_ANIM_STRIP_PATH, GOBLIN_RUN_ANIM_TEXTURE_ATLAS_INDICES,
    components::{Goblin, GoblinState},
};

pub fn setup_goblins(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    goblin_query: Query<Entity, Added<Goblin>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for goblin in goblin_query {
        let image = asset_server.load(GOBLIN_IDLE_ANIM_STRIP_PATH);

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.entity(goblin).insert((
            Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            AnimationTimer::default(),
            RigidBody::Dynamic,
            Collider::capsule_y(HALF_TILE_SIZE - 5.0, 6.0),
            LockedAxes::ROTATION_LOCKED,
            GOBLIN_IDLE_ANIM_TEXTURE_ATLAS_INDICES,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            Enemy,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

pub fn goblin_follow_player(
    goblin_query: Query<
        (Entity, &mut Goblin, &Transform, &mut Velocity),
        (With<Goblin>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Goblin>)>,
    mut enemy_triggered_event_writer: EventWriter<EnemyTriggeredEvent>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (goblin_entity, mut goblin, goblin_transform, mut goblin_velocity) in goblin_query {
        let difference_from_player_to_goblin =
            (player_transform.translation.x - goblin_transform.translation.x).abs();

        // Only follow player if goblin in a specific range of player
        if difference_from_player_to_goblin < 200.0 {
            // if now in range of player, and we previously had idle, then write EnemyTriggeredEvent
            if goblin.state == GoblinState::Idle {
                enemy_triggered_event_writer.write(EnemyTriggeredEvent {
                    enemy_entity: goblin_entity,
                });
            }

            if goblin_transform.translation.x > player_transform.translation.x {
                goblin_velocity.linvel.x = -50.0;
                // to avoid triggering Changed<>
                if goblin.state != GoblinState::RunBackwards {
                    goblin.state = GoblinState::RunBackwards;
                }
            } else if goblin_transform.translation.x < player_transform.translation.x {
                goblin_velocity.linvel.x = 50.0;

                // to avoid triggering Changed<>
                if goblin.state != GoblinState::RunForward {
                    goblin.state = GoblinState::RunForward;
                }
            }
        } else {
            // to avoid triggering Changed<>
            if goblin.state != GoblinState::Idle {
                goblin.state = GoblinState::Idle;
            }
        }
    }
}

pub fn update_goblin_sprite_if_changed(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    goblin_query: Query<(Entity, &Goblin, &mut Sprite), Changed<Goblin>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (goblin_entity, goblin, mut goblin_sprite) in goblin_query {
        match goblin.state {
            GoblinState::Idle => {
                goblin_sprite.image = asset_server.load(GOBLIN_IDLE_ANIM_STRIP_PATH);

                let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                goblin_sprite.texture_atlas = Some(TextureAtlas {
                    index: GOBLIN_IDLE_ANIM_TEXTURE_ATLAS_INDICES.first,
                    layout: texture_atlas_layout,
                });

                commands
                    .entity(goblin_entity)
                    .insert(GOBLIN_IDLE_ANIM_TEXTURE_ATLAS_INDICES);
            }
            GoblinState::RunForward => {
                goblin_sprite.image = asset_server.load(GOBLIN_RUN_ANIM_STRIP_PATH);
                goblin_sprite.flip_x = false;

                let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                goblin_sprite.texture_atlas = Some(TextureAtlas {
                    index: GOBLIN_RUN_ANIM_TEXTURE_ATLAS_INDICES.first,
                    layout: texture_atlas_layout,
                });

                commands
                    .entity(goblin_entity)
                    .insert(GOBLIN_RUN_ANIM_TEXTURE_ATLAS_INDICES);
            }
            GoblinState::RunBackwards => {
                goblin_sprite.image = asset_server.load(GOBLIN_RUN_ANIM_STRIP_PATH);
                goblin_sprite.flip_x = true;

                let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                goblin_sprite.texture_atlas = Some(TextureAtlas {
                    index: GOBLIN_RUN_ANIM_TEXTURE_ATLAS_INDICES.first,
                    layout: texture_atlas_layout,
                });

                commands
                    .entity(goblin_entity)
                    .insert(GOBLIN_RUN_ANIM_TEXTURE_ATLAS_INDICES);
            }
        }
    }
}

pub fn handle_goblin_triggered_event(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut enemy_triggered_event_reader: EventReader<EnemyTriggeredEvent>,
    goblin_query: Query<(Entity, &Transform), With<Goblin>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for enemy_triggered_event in enemy_triggered_event_reader.read() {
        let Some(goblin) = goblin_query
            .iter()
            .find(|(e, _)| *e == enemy_triggered_event.enemy_entity)
        else {
            continue;
        };

        let texture_atlas_layout =
            TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 4, None, None);

        let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);

        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout_handle,
            index: 0,
        };

        commands.spawn((
            Sprite::from_atlas_image(
                asset_server.load(ENEMY_EXCLAMATION_MARK_ANIM_STRIP_PATH),
                texture_atlas,
            ),
            Transform {
                translation: Vec3 {
                    x: goblin.1.translation.x,
                    y: goblin.1.translation.y + 20.0,
                    z: 2.0,
                },
                scale: Vec3::splat(0.5),
                ..default()
            },
            EnemyTriggered {
                timer: Timer::from_seconds(0.8, TimerMode::Once),
                enemy_entity: goblin.0,
            },
            ENEMY_EXCLAMATION_MARK_ANIM_TEXTURE_ATLAS_INDICES,
            AnimationTimer::default(),
        ));
    }
}

pub fn handle_enemy_triggered_timer(
    mut commands: Commands,
    goblin_query: Query<(Entity, &mut EnemyTriggered), With<EnemyTriggered>>,
    time: Res<Time>,
) {
    for (entity, mut enemy_triggered) in goblin_query {
        enemy_triggered.timer.tick(time.delta());
        if enemy_triggered.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    HALF_TILE_SIZE, common::components::AnimationTimer, enemy::components::Enemy,
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
    goblin_query: Query<(&mut Goblin, &Transform, &mut Velocity), (With<Goblin>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Goblin>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut goblin, goblin_transform, mut goblin_velocity) in goblin_query {
        let difference_from_player_to_goblin =
            (player_transform.translation.x - goblin_transform.translation.x).abs();

        // Only follow player if goblin in a specific range of player
        if difference_from_player_to_goblin < 200.0 {
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

pub fn handle_goblin_change(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    goblin_query: Query<(Entity, &Goblin, &mut Sprite), Changed<Goblin>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (goblin_entity, goblin, mut goblin_sprite) in goblin_query {
        info!("Goblin has changed! state is now: {:?}", goblin.state);
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

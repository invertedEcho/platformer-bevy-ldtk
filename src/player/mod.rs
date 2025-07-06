use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};

use crate::{GRID_SIZE, walls::resources::LevelWalls};

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
struct PlayerBundle {
    player: Player,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                Update,
                (
                    process_player,
                    animate_sprite,
                    move_player_from_input,
                    translate_grid_coords_entities,
                ),
            );
    }
}

fn process_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    new_players: Query<(Entity, &Transform), Added<Player>>,
) {
    for (entity, transform) in new_players {
        let texture = asset_server.load("herochar/herochar_run_anim_strip_6.png");
        println!("Texture handle: {:?}", texture);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 5 };

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, 3.0),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let movement_direction = if input.just_pressed(KeyCode::KeyW) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::KeyA) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::KeyS) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::KeyD) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation = grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
            .extend(transform.translation.z)
    }
}

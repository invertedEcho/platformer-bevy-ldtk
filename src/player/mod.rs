use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, LdtkEntity, Bundle)]
#[from_entity_instance]
struct PlayerBundle {
    player: Player,
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
            .add_systems(Update, process_player)
            .add_systems(Update, (animate_sprite, jump_player));
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
        // Use only the subset of sprites in the sheet that make up the run animation
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
            RigidBody::Dynamic,
            Collider::cuboid(8.0, 8.0),
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            LockedAxes::ROTATION_LOCKED,
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

fn jump_player(
    mut player_velocity: Query<(&mut Velocity, &Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (mut velocity, transform) = player_velocity.single_mut().expect("Player exists");

        velocity.linvel = Vec2::new(0.0, 300.0);
    }
}

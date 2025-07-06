use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component, Default)]
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
            .add_systems(Startup, spawn_player_sprite)
            .add_systems(Update, animate_sprite);
    }
}

fn spawn_player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("herochar/herochar_run_anim_strip_6.png");
    println!("Texture handle: {:?}", texture);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation: Vec3::new(0.0, 0.0, 3.0),
            // scale: Vec3::splat(16.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerSprite,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
    ldtk_player_entity: Query<&Transform, (With<Player>, Without<PlayerSprite>)>,
    mut bevy_player: Query<&mut Transform, (With<PlayerSprite>, Without<Player>)>,
) {
    if let Ok(ldtk_player) = ldtk_player_entity.single() {
        let ldtk_player_entity_translation = ldtk_player.translation;
        let mut bevy_player = bevy_player.single_mut().expect("Bevy Player sprite exists");
        bevy_player.translation = Vec3::new(
            ldtk_player_entity_translation.x,
            ldtk_player_entity_translation.y,
            3.0,
        );
    }
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

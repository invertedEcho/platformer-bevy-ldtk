use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use ground::GroundPlugin;
use jumper::JumperPlugin;
use player::PlayerPlugin;
use wall::WallPlugin;

mod camera;
mod ground;
mod jumper;
mod player;
pub mod utils;
mod wall;

pub const TILE_SIZE: i32 = 16;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(JumperPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: asset_server.load("ldtk/ldtk.ldtk"),
        },
        ..default()
    });

    rapier_config
        .single_mut()
        .expect("RapierConfiguration exists and can be mutated")
        .gravity = Vec2::new(0.0, -1000.0);
}

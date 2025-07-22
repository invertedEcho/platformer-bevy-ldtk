use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use coins::CoinPlugin;
use enemy::EnemyPlugin;
use game_flow::GameFlowPlugin;
use hud::HudPlugin;
use player::PlayerPlugin;
use world::ground::GroundPlugin;
use world::help_sign::HelpSignPlugin;
use world::mushroom::MushroomPlugin;
use world::platform::PlatformPlugin;
use world::wall::WallPlugin;

mod camera;
pub mod coins;
pub mod common;
mod enemy;
mod game_flow;
pub mod game_font;
mod hud;
mod player;
pub mod utils;
pub mod world;

pub const TILE_SIZE: f32 = 16.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;

// TODO: Investigate resource TextureAtlasLayout, feel like its duplicated everywhere
// TODO: Switch to a different font, I really don't like the current one

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(LdtkPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PlatformPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(GameFlowPlugin)
        .add_plugins(CoinPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(MushroomPlugin)
        .add_plugins(HelpSignPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0));
    if cfg!(debug_assertions) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
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

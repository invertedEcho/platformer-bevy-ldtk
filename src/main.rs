use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use coins::CoinPlugin;
use enemy::EnemyPlugin;
use game_flow::GameFlowPlugin;
use ground_detection::GroundDetectionPlugin;
use hud::HudPlugin;
use parallax_background::ParallaxBackgroundPlugin;
use player::PlayerPlugin;
use state::GameState;
use ui::main_menu::MainMenuPlugin;
use ui::pause_menu::PauseMenuPlugin;
use world::ground::GroundPlugin;
use world::moving_platform::MovingPlatformPlugin;
use world::mushroom::MushroomPlugin;
use world::one_way_platform::OneWayPlatformPlugin;
use world::save_point::SavePointPlugin;
use world::spike::SpikePlugin;
use world::tutorial::TutorialPlugin;
use world::wall::WallPlugin;

mod camera;
pub mod coins;
pub mod common;
mod enemy;
pub mod font;
mod game_flow;
mod ground_detection;
mod hud;
pub mod parallax_background;
pub mod player;
mod state;
mod ui;
pub mod utils;
pub mod world;

pub const TILE_SIZE: f32 = 16.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;

const LEVEL_IIDS: [&str; 3] = [
    "c2d47272-3740-11f0-a891-85a44477d8cd",
    "dd949e20-5e50-11f0-a1b6-870a0a448448",
    "8c4e3870-5e50-11f0-96e5-652a67f12f06",
];

// TODO: Investigate TextureAtlasLayouts
// TODO: Check whether theres a better way than just adding a run_if(in_state(GameState::Running))
// basically in all plugins

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "A platformer".into(),
                    ..default()
                }),
                ..default()
            }),
    )
    .init_state::<GameState>()
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    // .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(LdtkPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(WallPlugin)
    .add_plugins(OneWayPlatformPlugin)
    .add_plugins(GroundPlugin)
    .add_plugins(GameFlowPlugin)
    .add_plugins(CoinPlugin)
    .add_plugins(HudPlugin)
    .add_plugins(MushroomPlugin)
    .add_plugins(TutorialPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(SavePointPlugin)
    .add_plugins(SpikePlugin)
    .add_plugins(MovingPlatformPlugin)
    .add_plugins(ParallaxBackgroundPlugin)
    .add_plugins(GroundDetectionPlugin)
    .add_plugins(MainMenuPlugin)
    .add_plugins(PauseMenuPlugin)
    .add_systems(Startup, setup);
    if cfg!(debug_assertions) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    mut ui_scale: ResMut<UiScale>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: asset_server.load("ldtk.ldtk"),
        },
        ..default()
    });

    rapier_config
        .single_mut()
        .expect("RapierConfiguration exists and can be mutated")
        .gravity = Vec2::new(0.0, -1000.0);
    ui_scale.0 = 2.0;
}

use crate::{font::FONT_PATH, player::heart::resources::PlayerHeartResource};
use bevy::prelude::*;

use crate::coins::resources::CoinResource;

use super::components::{CoinCounter, PlayerHeartChild, PlayerHeartHud};

const COIN_HUD_ASSET_PATH: &str = "hud elements/coins_hud.png";

const NORMAL_HUD_GAP: Val = Val::Px(2.0);
const ROOT_UI_PADDING: Val = Val::Px(4.0);

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_heart_resource: Res<PlayerHeartResource>,
) {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    commands
        .spawn((
            Node {
                padding: UiRect {
                    top: ROOT_UI_PADDING,
                    left: ROOT_UI_PADDING,
                    right: ROOT_UI_PADDING,
                    bottom: ROOT_UI_PADDING,
                },
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: NORMAL_HUD_GAP,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 4.0),
        ))
        .with_children(|parent| {
            // Hearts
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: NORMAL_HUD_GAP,
                        ..default()
                    },
                    PlayerHeartHud,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageNode {
                        image: asset_server.load("hud elements/lifes_icon.png"),
                        ..default()
                    });
                    for _ in 0..player_heart_resource.count {
                        parent.spawn((
                            ImageNode {
                                image: asset_server.load("hud elements/hearts_hud.png"),
                                ..default()
                            },
                            PlayerHeartChild,
                        ));
                    }
                });

            // Coins
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    column_gap: NORMAL_HUD_GAP,
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(ImageNode {
                        image: asset_server.load(COIN_HUD_ASSET_PATH),
                        ..default()
                    });
                    parent.spawn((
                        CoinCounter,
                        Text::new("0"),
                        TextFont::from_font(font).with_font_size(12.0),
                    ));
                });
        });
}

pub fn update_coin_counter(
    coin_resource: Res<CoinResource>,
    mut coin_counter_query: Query<&mut Text, With<CoinCounter>>,
) {
    if !coin_resource.is_changed() {
        return;
    }
    let Ok(mut coin_counter) = coin_counter_query.single_mut() else {
        eprintln!("Failed to find coin counter");
        return;
    };
    **coin_counter = coin_resource.count.to_string();
}

// TODO: Dont despawn them all but check if we hearts increased/decreased. decreased -> despawn
// count by diff,
// increased, spawn new by diff
pub fn update_player_heart_count(
    mut commands: Commands,
    player_heart_resource: Res<PlayerHeartResource>,
    player_heart_hud_query: Query<Entity, With<PlayerHeartHud>>,
    player_heart_children_query: Query<Entity, With<PlayerHeartChild>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player_heart_hud) = player_heart_hud_query.single() else {
        eprintln!("Exactly one player heart hud should exist");
        return;
    };

    for player_heart_child in player_heart_children_query {
        commands.entity(player_heart_child).despawn();
    }

    for _ in 0..player_heart_resource.count {
        commands.entity(player_heart_hud).with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: asset_server.load("hud elements/hearts_hud.png"),
                    ..default()
                },
                PlayerHeartChild,
            ));
        });
    }
}

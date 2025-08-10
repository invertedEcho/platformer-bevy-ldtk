use crate::font::FONT_PATH;
use bevy::prelude::*;

use crate::coins::resources::CoinResource;

use super::components::CoinCounter;

const COIN_HUD_ASSET_PATH: &str = "hud elements/coins_hud.png";

const NORMAL_HUD_GAP: Val = Val::Px(2.0);
const ROOT_UI_PADDING: Val = Val::Px(4.0);

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    coin_resource: Res<CoinResource>,
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
            // Player Icon
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: NORMAL_HUD_GAP,
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(ImageNode {
                        image: asset_server.load("hud elements/lifes_icon.png"),
                        ..default()
                    });
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
                        Text::new(coin_resource.count.to_string()),
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
        error!("Failed to find coin counter");
        return;
    };
    info!(
        "coin_counter from HUD was updated to reflect the updated coin_resource. New count: {}",
        coin_resource.count
    );
    **coin_counter = coin_resource.count.to_string();
}

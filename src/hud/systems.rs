use crate::{game_font::FONT_ASSET_PATH, player::heart::resources::PlayerHeartResource};
use bevy::prelude::*;

use crate::coins::resources::CoinResource;

use super::components::{CoinCounterChild, CoinCounterHud, PlayerHeartChild, PlayerHeartHud};

const COIN_HUD_ASSET_PATH: &str = "hud elements/coins_hud.png";

const NORMAL_HUD_PARENT_HEIGHT: Val = Val::Px(30.0);
const NORMAL_HUD_GAP: Val = Val::Px(3.0);
const NORMAL_PADDING: Val = Val::Px(8.0);

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_heart_resource: Res<PlayerHeartResource>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(7), 10, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .spawn((
            Node {
                padding: UiRect {
                    top: NORMAL_PADDING,
                    left: NORMAL_PADDING,
                    right: NORMAL_PADDING,
                    bottom: NORMAL_PADDING,
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
                        height: NORMAL_HUD_PARENT_HEIGHT,
                        flex_direction: FlexDirection::Row,
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
                .spawn((
                    Node {
                        height: NORMAL_HUD_PARENT_HEIGHT,
                        flex_direction: FlexDirection::Row,
                        column_gap: NORMAL_HUD_GAP,
                        ..default()
                    },
                    CoinCounterHud,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageNode {
                        image: asset_server.load(COIN_HUD_ASSET_PATH),
                        ..default()
                    });
                    parent.spawn((
                        CoinCounterChild,
                        ImageNode::from_atlas_image(
                            asset_server.load(FONT_ASSET_PATH),
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                        ),
                    ));
                });
        });
}

pub fn update_coin_counter(
    mut commands: Commands,
    coin_resource: Res<CoinResource>,
    coin_counter_hud_query: Query<Entity, With<CoinCounterHud>>,
    coin_counter_children_query: Query<Entity, With<CoinCounterChild>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !coin_resource.is_changed() {
        return;
    }
    let coin_counter_hud_entity = coin_counter_hud_query
        .single()
        .expect("coin counter hud exists");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(7), 10, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for child in coin_counter_children_query {
        commands.entity(child).despawn();
    }

    for coin in coin_resource.count.to_string().chars() {
        let digit = coin.to_digit(10).unwrap();

        commands
            .entity(coin_counter_hud_entity)
            .with_children(|parent| {
                parent.spawn((
                    ImageNode::from_atlas_image(
                        asset_server.load(FONT_ASSET_PATH),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            // digit is actually the index in the texture atlas too
                            index: digit as usize,
                        },
                    ),
                    CoinCounterChild,
                ));
            });
    }
}

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

use crate::game_font::FONT_ASSET_PATH;
use bevy::prelude::*;

use crate::coins::resources::CoinResource;

use super::components::{CoinCounterChild, CoinCounterHud};

const COIN_HUD_ASSET_PATH: &str = "hud elements/coins_hud.png";

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(7), 10, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 4.0),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        height: Val::Px(20.0),
                        width: Val::Px(1000.0),
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

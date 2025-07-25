use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::font::{FONT_PATH, FONT_SIZE, KEYBOARD_TILESET_PATH};

use super::components::{KeyboardTile, TutorialText};

const TUTORIAL_TEXT_FIELD_IDENTIFIER: &str = "Tutorial_Text";

const KEYBOARD_TILE_KEY_CODE_FIELD_IDENTIFIER: &str = "KeyCode";
const KEYBOARD_TILE_TILE_FIELD_IDENTIFIER: &str = "KeyboardTile";
const KEYBOARD_TILE_SIZE: i32 = 16;

const PRESSED_KEYBOARD_TILE_FIELD_IDENTIFIER: &str = "PressedKeyboardTile";

pub fn spawn_text_for_tutorial_text(
    mut commands: Commands,
    tutorial_text_query: Query<(Entity, &EntityInstance, &mut Transform), Added<TutorialText>>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    for (entity, ldtk_entity, mut transform) in tutorial_text_query {
        let Ok(tutorial_text) = ldtk_entity.get_string_field(TUTORIAL_TEXT_FIELD_IDENTIFIER) else {
            eprintln!("Couldnt find tutorial text for ldtk entity");
            continue;
        };

        commands.entity(entity).insert((
            Text2d::new(tutorial_text),
            TextFont {
                font: font.clone(),
                font_size: FONT_SIZE,
                line_height: bevy::text::LineHeight::Px(20.0),
                ..default()
            },
            TextLayout {
                justify: JustifyText::Left,
                ..default()
            },
        ));

        // See: https://github.com/bevyengine/bevy/discussions/11443
        transform.scale = Vec3::splat(0.5);
    }
}

pub fn spawn_keyboard_tiles(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    keyboard_tile_query: Query<(Entity, &EntityInstance), Added<KeyboardTile>>,
) {
    for (entity, entity_instance) in keyboard_tile_query {
        let tile = entity_instance
            .get_tile_field(KEYBOARD_TILE_TILE_FIELD_IDENTIFIER)
            .expect("Keyboard tile has tile field");

        let image = asset_server.load(KEYBOARD_TILESET_PATH);

        commands.entity(entity).insert(Sprite {
            rect: Some(Rect {
                min: Vec2 {
                    x: tile.x as f32,
                    y: tile.y as f32,
                },
                max: Vec2 {
                    x: (tile.x + KEYBOARD_TILE_SIZE) as f32,
                    y: (tile.y + KEYBOARD_TILE_SIZE) as f32,
                },
            }),
            image,
            ..default()
        });
    }
}

fn get_key_code_identifier_for_key_code(key_code: &KeyCode) -> Option<&str> {
    match key_code {
        KeyCode::KeyS => Some("KeyS"),
        KeyCode::KeyD => Some("KeyD"),
        KeyCode::KeyA => Some("KeyA"),
        KeyCode::Space => Some("Space"),
        _ => None,
    }
}

pub fn change_keyboard_tiles(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    level_selection: Res<LevelSelection>,
    keyboard_tile_query: Query<(Entity, &EntityInstance), With<KeyboardTile>>,
) {
    if *level_selection
        != LevelSelection::Iid("c2d47272-3740-11f0-a891-85a44477d8cd".to_string().into())
    {
        return;
    };

    let just_pressed_keys: Vec<&KeyCode> = keyboard_input.get_just_pressed().collect();

    for just_pressed_key in just_pressed_keys {
        let Some(key_code_identifier) = get_key_code_identifier_for_key_code(just_pressed_key)
        else {
            continue;
        };
        let keyboard_tiles: Vec<(Entity, &EntityInstance)> = keyboard_tile_query
            .iter()
            .filter(|(_, entity_instance)| {
                key_code_identifier.to_string()
                    == *entity_instance
                        .get_string_field(KEYBOARD_TILE_KEY_CODE_FIELD_IDENTIFIER)
                        .expect("LDTK: KeyboardTile Tile field correctly typed")
            })
            .collect();

        for (entity, entity_instance) in keyboard_tiles {
            let pressed_keyboard_tile = entity_instance
                .get_tile_field(PRESSED_KEYBOARD_TILE_FIELD_IDENTIFIER)
                .expect("LDTK: KeyboardTile KeyCode field correctly typed");
            commands.entity(entity).insert(Sprite {
                image: asset_server.load(KEYBOARD_TILESET_PATH),
                rect: Some(Rect {
                    min: Vec2 {
                        x: pressed_keyboard_tile.x as f32,
                        y: pressed_keyboard_tile.y as f32,
                    },
                    max: Vec2 {
                        x: (pressed_keyboard_tile.x + KEYBOARD_TILE_SIZE) as f32,
                        y: (pressed_keyboard_tile.y + KEYBOARD_TILE_SIZE) as f32,
                    },
                }),
                ..default()
            });
        }
    }

    let just_released_keys: Vec<&KeyCode> = keyboard_input.get_just_released().collect();

    for just_released_key in just_released_keys {
        let Some(key_code_identifier) = get_key_code_identifier_for_key_code(just_released_key)
        else {
            continue;
        };
        let keyboard_tiles: Vec<(Entity, &EntityInstance)> = keyboard_tile_query
            .iter()
            .filter(|(_, entity_instance)| {
                key_code_identifier.to_string()
                    == *entity_instance
                        .get_string_field(KEYBOARD_TILE_KEY_CODE_FIELD_IDENTIFIER)
                        .expect("LDTK: KeyboardTile Tile field correctly typed")
            })
            .collect();

        for (entity, entity_instance) in keyboard_tiles {
            let released_keyboard_tile = entity_instance
                .get_tile_field(KEYBOARD_TILE_TILE_FIELD_IDENTIFIER)
                .expect("LDTK: KeyboardTile KeyCode field correctly typed");
            commands.entity(entity).insert(Sprite {
                image: asset_server.load(KEYBOARD_TILESET_PATH),
                rect: Some(Rect {
                    min: Vec2 {
                        x: released_keyboard_tile.x as f32,
                        y: released_keyboard_tile.y as f32,
                    },
                    max: Vec2 {
                        x: (released_keyboard_tile.x + KEYBOARD_TILE_SIZE) as f32,
                        y: (released_keyboard_tile.y + KEYBOARD_TILE_SIZE) as f32,
                    },
                }),
                ..default()
            });
        }
    }
}

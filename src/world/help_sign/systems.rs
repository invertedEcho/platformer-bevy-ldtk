use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game_font::{
    FONT_ASSET_PATH, FONT_GLYPH_SIZE, FONT_SPACEBAR_INDEX, get_font_char_index,
    get_font_indices_from_text,
};

use super::components::HelpSign;

const HELP_SIGN_ENUM_IDENTIFIER: &str = "HelpSigns";

const CELL: Vec2 = Vec2::new(FONT_GLYPH_SIZE as f32, FONT_GLYPH_SIZE as f32);
const LINE_WIDTH: f32 = 15.0 * CELL.x;

pub fn spawn_help_text_for_help_signs(
    mut commands: Commands,
    help_signs_query: Query<(Entity, &EntityInstance), Added<HelpSign>>,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let font_texture = asset_server.load(FONT_ASSET_PATH);
    let atlas_handle = {
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(FONT_GLYPH_SIZE), 10, 5, None, None);
        atlases.add(layout)
    };

    for (entity, ldtk_entity) in help_signs_query {
        let mut x_cursor = 0.0;
        let mut y_cursor = 0.0;

        let Ok(help_sign_enum_field) = ldtk_entity.get_enum_field(HELP_SIGN_ENUM_IDENTIFIER) else {
            eprintln!("Couldnt find enum field from entity");
            continue;
        };

        let Ok(help_str) = get_help_text_from_help_sign_field(help_sign_enum_field) else {
            eprintln!(
                "Couldnt find help text for help sign field: {}",
                help_sign_enum_field
            );
            continue;
        };

        commands
            .entity(entity)
            .insert(Visibility::Visible)
            .with_children(|parent| {
                for (index, font_index) in get_font_indices_from_text(&help_str).iter().enumerate()
                {
                    let Some(next_char) = help_str.chars().nth(index + 1) else {
                        println!("No more next chars, end of text. just spawning normal character and then end");
                        parent.spawn((
                            Sprite::from_atlas_image(
                                font_texture.clone(),
                                TextureAtlas {
                                    layout: atlas_handle.clone(),
                                    index: *font_index,
                                },
                            ),
                            Transform::from_translation(Vec3::new(x_cursor, y_cursor, 0.0)),
                        ));
                        continue;
                    };

                    let next_char_font_index = get_font_char_index(&next_char).unwrap();
                    println!("Next char: {}", next_char);
                    println!("next char font index: {}", next_char_font_index);

                    // if current font index is space, add empty gap, but only do so if we didnt
                    // just line wrap.
                    if *font_index == FONT_SPACEBAR_INDEX {
                        let x_cursor_at_line_start = x_cursor == 0.0;
                        if x_cursor_at_line_start {
                            continue;
                        }
                        x_cursor += CELL.x / 2.0 + 1.0;
                        println!("current font index is space, adding empty gap and skipping to next iteration of for loop");
                        println!("\n");
                        continue;
                    } else {
                        println!("Spawning character: {}", help_str.chars().nth(index).unwrap());

                        parent.spawn((
                            Sprite::from_atlas_image(
                                font_texture.clone(),
                                TextureAtlas {
                                    layout: atlas_handle.clone(),
                                    index: *font_index,
                                },
                            ),
                            Transform::from_translation(Vec3::new(x_cursor, y_cursor, 0.0)),
                        ));
                        x_cursor += CELL.x + 1.0;
                    }

                    // If we just wrote a character, and it exceeded LINE_WIDTH, decrease y so its wrapped
                    // But only wrap if next char is a space
                    if x_cursor > LINE_WIDTH && next_char_font_index == FONT_SPACEBAR_INDEX
                    {
                        println!("next_char_font is space, and adding char would exceed LINE_WIDTH. Line wrapping, and continuing.");
                        x_cursor = 0.0;
                        y_cursor -= CELL.y + 2.0;
                    }


                    println!("\n");
                }
            });
    }
}

fn get_help_text_from_help_sign_field(value: &String) -> Result<String, &str> {
    let basic_move_string = String::from("Basic_Move");
    let jump_string = String::from("Jump");
    let platform_string = String::from("Platform");
    if *value == basic_move_string {
        return Ok("Use D to move forward and A to move backwards".to_string());
    } else if *value == jump_string {
        return Ok("Use the Spacebar to jump".to_string());
    } else if *value == platform_string {
        return Ok("Use S to fall through a platform".to_string());
    } else {
        return Err("Invalid help sign value");
    }
}

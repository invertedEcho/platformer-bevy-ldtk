use bevy::{
    prelude::*,
    text::{FontSmoothing, TextBounds},
};
use bevy_ecs_ldtk::prelude::*;

use crate::font::{FONT_PATH, FONT_SIZE};

use super::components::HelpSign;

const HELP_SIGN_ENUM_IDENTIFIER: &str = "HelpSigns";

pub fn spawn_help_text_for_help_signs(
    mut commands: Commands,
    help_signs_query: Query<(Entity, &EntityInstance, &mut Transform), Added<HelpSign>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(FONT_PATH);

    for (entity, ldtk_entity, mut transform) in help_signs_query {
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

        commands.entity(entity).insert((
            TextBounds {
                width: Some(250.0),
                ..default()
            },
            Text2d::new(help_str),
            TextFont {
                font: font.clone(),
                font_size: FONT_SIZE,
                font_smoothing: FontSmoothing::AntiAliased,
                ..default()
            },
            TextLayout {
                linebreak: LineBreak::WordBoundary,
                justify: JustifyText::Center,
            },
        ));
        // See: https://github.com/bevyengine/bevy/discussions/11443
        transform.scale = Vec3::splat(0.5);
    }
}

fn get_help_text_from_help_sign_field(value: &String) -> Result<&str, &str> {
    let basic_move_string = String::from("Basic_Move");
    let jump_string = String::from("Jump");
    let platform_string = String::from("Platform");
    let slime_string = String::from("Slimes");
    let next_level_orb = String::from("NextLevelOrb");

    if *value == basic_move_string {
        return Ok("Use D to move forward and A to move backwards");
    } else if *value == jump_string {
        return Ok("Use the Spacebar to jump");
    } else if *value == platform_string {
        return Ok("Use S to fall through a platform");
    } else if *value == slime_string {
        return Ok("Watch out for Slimes, they will hurt you!");
    } else if *value == next_level_orb {
        return Ok("This orb will get you to the next level");
    } else {
        return Err("Invalid help sign value");
    }
}

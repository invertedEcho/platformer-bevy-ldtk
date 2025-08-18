use bevy::prelude::*;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

use bevy::log::info;

use crate::game_save::{components::GameSaveTextTimer, migration::migrate_game_save};

use super::{GAME_SAVE_FILE_PATH, GameSave};

pub fn get_or_create_game_save() -> GameSave {
    let game_save_file_exists =
        fs::exists(GAME_SAVE_FILE_PATH).expect("Can check if GAME_SAVE_FILE_PATH exists");

    if game_save_file_exists {
        let mut game_save_file =
            File::open("game_save.json").expect("Can open game_save.json if file exists");

        let mut file_buffer = String::from("");

        let result = game_save_file.read_to_string(&mut file_buffer);
        match result {
            Ok(_) => info!("Sucessfully read from game save file into buffer"),
            Err(err) => panic!("Failed to read from game save file into buffer: {}", err),
        }

        let game_save: Result<GameSave, serde_json::error::Error> =
            serde_json::from_str(&file_buffer);
        match game_save {
            Ok(game_save) => {
                println!("Sucessfully serialized existing game save to GameSave struct.");
                return game_save;
            }
            Err(error) => {
                error!(
                    "Failed to parse game save json str into rust struct.: {}",
                    error
                );
                info!("Trying to fix game save file by checking for game save version");
                let migrate_result = migrate_game_save(&mut file_buffer);
                match migrate_result {
                    Ok(res) => res,
                    Err(res) => {
                        panic!("{}", res);
                    }
                }
            }
        }
    } else {
        let game_save: GameSave = GameSave::default();

        fs::write(
            GAME_SAVE_FILE_PATH,
            serde_json::to_vec(&game_save).expect("Can serialize GameSave to vec"),
        )
        .expect("Can create initial game save file with initial content");

        println!("Created initial game save.");

        return game_save;
    }
}

pub fn update_game_save(new_game_save: &GameSave) {
    let write_result = File::create(GAME_SAVE_FILE_PATH)
        .expect("Can create game save file")
        .write_all(&serde_json::to_vec(&new_game_save).expect("Can serialize to json string"));

    match write_result {
        Ok(()) => {
            // TODO: reintroduce this by seperating it into a seperate system which reacts on an
            // event, called smth like PostResetGameSaveEvent::Success
            // commands.spawn((
            //     Text::new("Game Saved!"),
            //     GameSaveTextTimer(Timer::from_seconds(3.0, TimerMode::Once)),
            // ));
            info!("Sucessfully updated game save!");
        }
        Err(err) => {
            panic!("Failed to update game save: {}", err);
        }
    }
}

pub fn handle_game_save_text_timer(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &mut GameSaveTextTimer)>,
) {
    for (entity, mut timer) in query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn reset_game_save() {
    update_game_save(&GameSave::default());
}

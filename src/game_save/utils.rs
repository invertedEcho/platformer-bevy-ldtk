use bevy::prelude::*;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

use bevy::log::info;

use crate::{INITIAL_LEVEL_IID, game_save::components::GameSaveTextTimer};

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

        let game_save: GameSave = serde_json::from_str(&file_buffer).expect("JSON parse error");
        println!("Sucessfully serialized existing game save to GameSave struct.");
        return game_save;
    } else {
        let game_save: GameSave = GameSave {
            level_iid: INITIAL_LEVEL_IID.to_string(),
            player_coins: 0,
        };

        fs::write(
            GAME_SAVE_FILE_PATH,
            serde_json::to_vec(&game_save).expect("Can serialize GameSave to vec"),
        )
        .expect("Can create initial game save file with initial content");

        println!("Created initial game save.");

        return game_save;
    }
}

pub fn update_game_save(commands: &mut Commands, new_game_save: GameSave) {
    let write_result = File::create(GAME_SAVE_FILE_PATH)
        .expect("Can create game save file")
        .write_all(&serde_json::to_vec(&new_game_save).expect("Can serialize to json string"));

    match write_result {
        Ok(()) => {
            commands.spawn((
                Text::new("Game Saved!"),
                GameSaveTextTimer(Timer::from_seconds(3.0, TimerMode::Once)),
            ));
            println!("updated player coins to 5");
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

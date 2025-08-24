use std::ops::Neg;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ground_detection::components::GroundDetection,
    player::components::{Player, PlayerDirection, PlayerState},
};

use super::{PLAYER_JUMP_NORMAL, PLAYER_SPEED};

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Player, &GroundDetection), With<Player>>,
) {
    for (mut velocity, mut player, ground_detection) in player_query.iter_mut() {
        if player.state == PlayerState::Dead {
            continue;
        }
        if input.pressed(KeyCode::KeyD) {
            velocity.linvel.x = PLAYER_SPEED;
            if player.direction != PlayerDirection::Forward {
                player.direction = PlayerDirection::Forward;
            }
            if player.state != PlayerState::Run {
                player.state = PlayerState::Run;
            }
        }
        if input.pressed(KeyCode::KeyA) {
            velocity.linvel.x = PLAYER_SPEED.neg();
            if player.direction != PlayerDirection::Backwards {
                player.direction = PlayerDirection::Backwards;
            }
            if player.state != PlayerState::Run {
                player.state = PlayerState::Run;
            }
        }
        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = PLAYER_JUMP_NORMAL;
            player.state = PlayerState::Jump;
        }
        let player_just_stopped_moving = input.just_released(KeyCode::KeyD)
            || input.just_released(KeyCode::KeyA)
            || input.just_released(KeyCode::Space);
        if player_just_stopped_moving {
            player.state = PlayerState::Idle;
            velocity.linvel.x = 0.0;
        }
    }
}

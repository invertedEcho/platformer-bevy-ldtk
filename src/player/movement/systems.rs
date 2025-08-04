use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::components::Player;

use super::{PLAYER_JUMP_NORMAL, PLAYER_SPEED, states::PlayerMovementType};

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Player), With<Player>>,
    current_player_movement_type: Res<State<PlayerMovementType>>,
    mut next_player_movement_type: ResMut<NextState<PlayerMovementType>>,
) {
    for (mut velocity, mut player) in player_query.iter_mut() {
        if !player.on_horizontal_moving_platform {
            velocity.linvel.x = 0.0;
        }

        let current_player_movement_type = current_player_movement_type.get().clone();
        if current_player_movement_type == PlayerMovementType::BackwardsRun
            || current_player_movement_type == PlayerMovementType::BackwardsIdle
        {
            next_player_movement_type.set(PlayerMovementType::BackwardsIdle);
        } else {
            next_player_movement_type.set(PlayerMovementType::ForwardIdle);
        }
        if input.pressed(KeyCode::KeyD) {
            velocity.linvel.x = 1.0 * PLAYER_SPEED;
            next_player_movement_type.set(PlayerMovementType::ForwardRun);
        }
        if input.pressed(KeyCode::KeyA) {
            velocity.linvel.x = -1.0 * PLAYER_SPEED;
            next_player_movement_type.set(PlayerMovementType::BackwardsRun);
        }
        if input.just_pressed(KeyCode::Space) && !player.jumping {
            velocity.linvel.y = PLAYER_JUMP_NORMAL;
            player.jumping = true;
        }
    }
}

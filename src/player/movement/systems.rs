use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::components::Player,
    world::platform::components::{Platform, PlatformCollidingWithPlayer},
};

use super::{PLAYER_JUMP_NORMAL, PLAYER_SPEED, states::PlayerMovementType};

pub fn player_movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Player), With<Player>>,
    current_player_movement_type: Res<State<PlayerMovementType>>,
    mut next_player_movement_type: ResMut<NextState<PlayerMovementType>>,
    platform_query: Query<Entity, (With<Platform>, With<PlatformCollidingWithPlayer>)>,
) {
    for (mut velocity, mut player) in player_query.iter_mut() {
        velocity.linvel.x = 0.0;
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
        if input.just_pressed(KeyCode::KeyS) && !player.is_jumping {
            for platform_entity in platform_query {
                println!("Inserting collider disabled bcs pressed KeyS");
                commands.entity(platform_entity).insert(ColliderDisabled);
            }
        }
        if input.just_pressed(KeyCode::Space) && !player.is_jumping {
            velocity.linvel.y = PLAYER_JUMP_NORMAL;
            player.is_jumping = true;
        }
    }
}

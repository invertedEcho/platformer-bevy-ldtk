use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::coins::resources::CoinResource;

pub fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
    input: Res<ButtonInput<KeyCode>>,
    mut coin_resource: ResMut<CoinResource>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        commands
            .entity(
                ldtk_projects
                    .single()
                    .expect("Exactly one ldtk project exists"),
            )
            .insert(Respawn);
        coin_resource.count = 0;
    }
}

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn respawn_world(
    mut commands: Commands,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        commands
            .entity(
                ldtk_projects
                    .single()
                    .expect("Exactly one ldtk project exists"),
            )
            .insert(Respawn);
    }
}

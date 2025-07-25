use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct SavePoint;

#[derive(Bundle, Default, LdtkEntity)]
pub struct SavePointBundle {
    save_point: SavePoint,
}

#[derive(Component, Deref, DerefMut)]
pub struct SavingSavePointTimer(pub Timer);

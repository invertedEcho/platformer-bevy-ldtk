use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Slime;

// need seperate components as slime is bobbing, so physic and visual need to be seperated
#[derive(Component)]
pub struct SlimeSprite;

#[derive(LdtkEntity, Default, Bundle)]
pub struct SlimeBundle {
    enemy: Slime,
}

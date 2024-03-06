use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Damage {
    pub value: i32,
}

use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub owner: usize,
}

#[derive(Component)]
pub struct Damage {
    pub value: i32,
}

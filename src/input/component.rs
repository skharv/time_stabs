use bevy::prelude::*;

#[derive(Component)]
pub struct Mouse;

#[derive(Component)]
pub struct ClickPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct ClickTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

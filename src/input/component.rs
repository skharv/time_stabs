use bevy::prelude::*;

#[derive(Component)]
pub struct Mouse;

#[derive(Component)]
pub struct ClickPosition {
    pub x: f32,
    pub y: f32,
}

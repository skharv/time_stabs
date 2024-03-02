use bevy::prelude::*;

pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

#[derive(Component)]
pub struct Mouse;

#[derive(Component)]
pub struct ClickPosition {
    pub x: f32,
    pub y: f32,
}

impl AsVec2 for ClickPosition {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
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

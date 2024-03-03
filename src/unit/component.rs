use std::collections::VecDeque;

use bevy::prelude::*;

use super::action::Action;
use super::history::Snapshot;

pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

#[derive(Component)]
pub struct Radius{
    pub value: f32
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Ghost;

#[derive(Component)]
pub struct Facing {
    pub value: f32
}

#[derive(Component)]
pub struct State {
    pub value: Action
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl AsVec2 for Velocity {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Component)]
pub struct TurnRate {
    pub value: f32
}

#[derive(Component)]
pub struct MoveSpeed {
    pub value: f32
}

#[derive(Component)]
pub struct Target {
    pub x: f32,
    pub y: f32
}

impl AsVec2 for Target {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Component)]
pub struct History {
    pub snapshots: VecDeque<Snapshot>
}

#[derive(Component)]
pub struct Repeat {
    pub timestamp: f32
}

#[derive(Component)]
pub struct Reverse {
    pub timestamp: f32
}

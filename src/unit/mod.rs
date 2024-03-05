use std::{collections::VecDeque, f32::consts::PI};
use bevy::prelude::*;
use crate::input::component::{Selectable, Selected};

pub mod action;
mod animation;
pub mod component;
mod collision;
mod history;
mod movement;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Idle,
    Move,
    AttackMove,
    Attack,
    Stop,
    Halt,
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn, spawn_enemy))
            .add_systems(Update, (
                    show_selection,
                    animation::animate_texture_atlas,
                    action::read_action,
                    action::engage,
                    history::start_repeat,
                    history::start_reverse,
                    movement::arrive,
                    movement::apply_velocity.after(collision::collision),
                    movement::calculate_direct_velocity.after(movement::turn_towards_target),
                    movement::turn_towards_target,
                    collision::collision.after(movement::calculate_direct_velocity),
                    ))
            .add_systems(FixedUpdate, (
                    action::attack,
                    history::track_history,
                    history::repeat_history,
                    ));
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
    let unit_radius = 25.0;
    for n in 0..10 {
        let texture = asset_server.load::<Image>("marine.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 8, 8, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let spawn_transform = Transform::from_xyz((n * 30) as f32, (n * 30) as f32, 0.0);
        let parent = commands.spawn((
                SpriteSheetBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    },
                    transform: spawn_transform,
                    ..default()
                },
                Selectable,
                component::Unit{ owner: 0 },
                component::Radius { value: unit_radius },
                component::Velocity { x: 0.0, y: 0.0 },
                component::MoveSpeed { value: 200.0 },
                component::Facing { value: (2.0 * PI / 10.0) * n as f32 },
                component::TurnRate { value: 10.0 },
                component::Target { entity: None, x: 0.0, y: 0.0 },
                component::CurrentAction { value: action::Action::None },
                component::Attack { range: 500.0, timer: Timer::from_seconds(5.0, TimerMode::Once) },
                component::CurrentState { value: State::Idle },
                component::History { snapshots: VecDeque::new() },
                component::AnimationIndices { current: 0, first: 0, last: 7 },
                component::AnimationTimer { timer: Timer::from_seconds(0.08, TimerMode::Repeating) },
                )).id();

        let child_texture = asset_server.load::<Image>("selection_circle.png");
        let child = commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                texture: child_texture,
                transform: Transform::from_xyz(0.0, -25.0, -100.0),
                visibility: Visibility::Hidden,
                ..default()
            }).id();

        commands.entity(parent).add_child(child);
    }
}

pub fn show_selection(
    unit_query: Query<(&Children, Option<&Selected>), With<component::Unit>>,
    mut child_query: Query<&mut Visibility, Without<component::Unit>>,
    ) {
    for (children, opt_selected) in unit_query.iter() {
        for &child in children.iter() {
            if let Ok(mut visibility) = child_query.get_mut(child) {
                if let Some(_) = opt_selected {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
    let unit_radius = 25.0;
    let texture = asset_server.load::<Image>("marine.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let spawn_transform = Transform::from_xyz(-100.0, -100.0, 0.0);
    commands.spawn((
            SpriteSheetBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
                transform: spawn_transform,
                ..default()
            },
            component::Unit { owner: 1 },
            component::Enemy,
            component::Radius { value: unit_radius },
            component::Velocity { x: 0.0, y: 0.0 },
            component::MoveSpeed { value: 200.0 },
            component::Facing { value: 0.0 },
            component::TurnRate { value: 10.0 },
            component::Target { entity: None, x: 0.0, y: 0.0 },
            component::CurrentAction { value: action::Action::None },
            component::Attack { range: 500.0, timer: Timer::from_seconds(5.0, TimerMode::Once) },
            component::CurrentState { value: State::Idle },
            component::History { snapshots: VecDeque::new() },
            component::AnimationIndices { current: 0, first: 0, last: 7 },
            component::AnimationTimer { timer: Timer::from_seconds(0.08, TimerMode::Repeating) },
            ));

}

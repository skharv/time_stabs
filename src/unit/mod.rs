use std::{collections::VecDeque, f32::consts::PI};
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::input::component::{Selectable, Selected};
use rand::Rng;

use crate::AppState;

pub mod action;
mod animation;
pub mod component;
mod collision;
mod history;
mod movement;

pub const HEALTH_BAR_HEIGHT: f32 = 6.0;
pub const HEALTH_BAR_WIDTH: f32 = 50.0;
pub const HEALTH_BAR_BORDER: f32 = 2.0;

pub const UNIT_RADIUS: f32 = 20.0;
pub const UNIT_MOVE_SPEED: f32 = 200.0;
pub const UNIT_TURN_RATE: f32 = 10.0;
pub const UNIT_ATTACK_RANGE: f32 = 500.0;
pub const UNIT_ATTACK_TIMER: f32 = 1.0;
pub const UNIT_HEALTH: i32 = 100;
pub const UNIT_ANIMATION_TIMER: f32 = 0.08;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Idle,
    Move,
    AttackMove,
    Attack,
    Dead,
    Stop,
    Halt,
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (spawn, spawn_enemy))
            .add_systems(Update, (
                    show_selection,
                    health_ui,
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
                    health.after(action::attack),
                    history::track_history,
                    history::repeat_history,
                    ));
    }
}

pub fn health(
    mut commands: Commands,
    mut dying_query: Query<(Entity, &mut Transform, &Children, &component::Health, &mut component::CurrentState, &mut TextureAtlas)>,
    mut target_query: Query<&mut component::Target, With<component::Unit>>,
    ) {
    for (entity, mut transform, children, health, mut state, mut atlas) in dying_query.iter_mut() {
        if health.current <= 0 {
            let mut rng = rand::thread_rng();
            for mut target in target_query.iter_mut() {
                if target.entity == Some(entity) {
                    target.entity = None;
                }
            }
            if state.value != State::Dead {
                state.value = State::Dead;
                transform.translation.z -= 200.0;
                atlas.index = rng.gen_range(80..83);
                for &child in children.iter() {
                    commands.entity(child).despawn_recursive();
                }
                commands.entity(entity).remove::<component::Velocity>();
                commands.entity(entity).remove::<component::Radius>();
                commands.entity(entity).remove::<component::MoveSpeed>();
                commands.entity(entity).remove::<component::Unit>();
            }
        }
    }
}

pub fn health_ui(
    mut query: Query<(&component::Health, &Children), With<component::Unit>>,
    mut bar_query: Query<(&mut Transform, &mut Visibility), (With<component::HealthBarAmountUi>, Without<component::HealthBarUi>)>,
    mut border_query: Query<(&mut Transform, &mut Visibility), (With<component::HealthBarUi>, Without<component::HealthBarAmountUi>)>,
    ) {
    for (health, children) in query.iter_mut() {
        if health.current == health.max {
            for &child in children.iter() {
                if let Ok((_, mut visibility)) = bar_query.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }
                if let Ok((_, mut visibility)) = border_query.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }
            }
        } else {
            for &child in children.iter() {
                if let Ok((_, mut visibility)) = bar_query.get_mut(child) {
                    *visibility = Visibility::Visible;
                }
                if let Ok((_, mut visibility)) = border_query.get_mut(child) {
                    *visibility = Visibility::Visible;
                }
            }
        }

        for &child in children.iter() {
            if let Ok((mut transform, _)) = bar_query.get_mut(child) {
                let scale = health.current as f32 / health.max as f32;
                transform.scale.x = scale;
                transform.translation.x = -(HEALTH_BAR_WIDTH / 2.0) + (HEALTH_BAR_WIDTH * scale / 2.0);
            }
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    for n in 0..10 {
        let texture = asset_server.load::<Image>("marine.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 8, 11, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let spawn_transform = Transform::from_xyz((n * 30) as f32, (n * 30) as f32, 0.0);
        let parent = commands.spawn(SpriteSheetBundle {
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
        }).id();

        commands.entity(parent).insert((
                Selectable,
                component::Unit{ owner: 0 },
                component::Radius { value: UNIT_RADIUS },
                component::Velocity { x: 0.0, y: 0.0 },
                component::MoveSpeed { value: UNIT_MOVE_SPEED },
                component::Facing { value: (2.0 * PI / 10.0) * n as f32 },
                component::TurnRate { value: UNIT_TURN_RATE },
                component::Target { entity: None, x: 0.0, y: 0.0 },
                component::CurrentAction { value: action::Action::None },
                component::Attack { range: UNIT_ATTACK_RANGE, timer: Timer::from_seconds(UNIT_ATTACK_TIMER, TimerMode::Once) },
                component::CurrentState { value: State::Idle },
                component::History { snapshots: VecDeque::new() },
                component::Health { current: UNIT_HEALTH, max: UNIT_HEALTH },
                component::AnimationIndices { current: 0, first: 0, last: 7 },
                component::AnimationTimer { timer: Timer::from_seconds(UNIT_ANIMATION_TIMER, TimerMode::Repeating) },
                ));

        let child_texture = asset_server.load::<Image>("selection_circle.png");
        let child = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    texture: child_texture,
                    transform: Transform::from_xyz(0.0, -25.0, -100.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                component::SelectionCircleUi
                )).id();

        commands.entity(parent).add_child(child);

        let outer_shape = Mesh2dHandle(meshes.add(Rectangle::new(HEALTH_BAR_WIDTH + HEALTH_BAR_BORDER, HEALTH_BAR_HEIGHT + HEALTH_BAR_BORDER)));
        let inner_shape = Mesh2dHandle(meshes.add(Rectangle::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)));
        let outer_color = Color::BLACK;
        let inner_color = Color::GREEN;

        let outer = commands.spawn((MaterialMesh2dBundle {
            mesh: outer_shape,
            material: materials.add(outer_color),
            transform: Transform::from_xyz(0.0, 50.0, 100.0),
            ..default()
        },
        component::HealthBarUi)).id();

        let inner = commands.spawn((MaterialMesh2dBundle {
            mesh: inner_shape,
            material: materials.add(inner_color),
            transform: Transform::from_xyz(0.0, 50.0, 101.0),
            ..default()
        },
        component::HealthBarAmountUi)).id();

        commands.entity(parent).add_child(outer);
        commands.entity(parent).add_child(inner);
    }
}

pub fn show_selection(
    unit_query: Query<(&Children, Option<&Selected>), With<component::Unit>>,
    mut child_query: Query<&mut Visibility, (With<component::SelectionCircleUi>, Without<component::Unit>)>,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    for n in 0..10 {
        let texture = asset_server.load::<Image>("marine.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(100.0, 100.0), 8, 11, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let spawn_transform = Transform::from_xyz(-100.0, -100.0, 0.0);
        let parent = commands.spawn(
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
            ).insert((
                    component::Unit { owner: 1 },
                    component::Enemy,
                    component::Radius { value: UNIT_RADIUS },
                    component::Velocity { x: 0.0, y: 0.0 },
                    component::MoveSpeed { value: UNIT_MOVE_SPEED },
                    component::TurnRate { value: UNIT_TURN_RATE },
                    component::Facing { value: 0.0 },
                    component::Target { entity: None, x: 0.0, y: 0.0 },
                    component::CurrentAction { value: action::Action::None },
                    component::Attack { range: UNIT_ATTACK_RANGE, timer: Timer::from_seconds(UNIT_ATTACK_TIMER, TimerMode::Once) },
                    component::CurrentState { value: State::Idle },
                    component::History { snapshots: VecDeque::new() },
                    component::Health { current: UNIT_HEALTH, max: UNIT_HEALTH },
                    component::AnimationIndices { current: 0, first: 0, last: 7 },
                    component::AnimationTimer { timer: Timer::from_seconds(UNIT_ANIMATION_TIMER, TimerMode::Repeating) },
                    )).id();


        let outer_shape = Mesh2dHandle(meshes.add(Rectangle::new(HEALTH_BAR_WIDTH + HEALTH_BAR_BORDER, HEALTH_BAR_HEIGHT + HEALTH_BAR_BORDER)));
        let inner_shape = Mesh2dHandle(meshes.add(Rectangle::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)));
        let outer_color = Color::BLACK;
        let inner_color = Color::GREEN;

        let outer = commands.spawn((MaterialMesh2dBundle {
            mesh: outer_shape,
            material: materials.add(outer_color),
            transform: Transform::from_xyz(0.0, 50.0, 100.0),
            ..default()
        },
        component::HealthBarUi)).id();

        let inner = commands.spawn((MaterialMesh2dBundle {
            mesh: inner_shape,
            material: materials.add(inner_color),
            transform: Transform::from_xyz(0.0, 50.0, 101.0),
            ..default()
        },
        component::HealthBarAmountUi)).id();

        commands.entity(parent).add_child(outer);
        commands.entity(parent).add_child(inner);
    }
}

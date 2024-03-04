use std::{collections::VecDeque, f32::consts::PI};
use bevy::prelude::*;
use crate::input::component::{Selectable, Selected};

pub mod action;
pub mod component;
mod collision;
mod history;
mod movement;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Idle,
    Move,
    Attack,
    Stop,
    Halt,
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (
                    draw_gizmo,
                    gizmo_config,
                    animate_sprite,
                    action::read_action,
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
        commands.spawn((
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
                component::Unit,
                component::Radius { value: unit_radius },
                component::Velocity { x: 0.0, y: 0.0 },
                component::MoveSpeed { value: 200.0 },
                component::Facing { value: (2.0 * PI / 10.0) * n as f32 },
                component::TurnRate { value: 10.0 },
                component::Target { x: 0.0, y: 0.0 },
                component::CurrentAction { value: action::Action::None },
                component::Attack { timer: Timer::from_seconds(5.0, TimerMode::Once) },
                component::CurrentState { value: State::Idle },
                component::History { snapshots: VecDeque::new() },
                component::AnimationIndices { current: 0, first: 0, last: 7 },
                component::AnimationTimer { timer: Timer::from_seconds(0.08, TimerMode::Repeating) },
                ));
    }
}

fn angle_to_direction(angle: f32) -> usize {
    let angle_positive = (angle + (2.0 * PI)) % (2.0 * PI);
    let index = ((angle_positive + PI / 8.0) % (2.0 * PI) / (PI / 4.0)) as usize;
    index
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut component::AnimationIndices, &mut component::AnimationTimer, &component::Facing, &component::CurrentState), (With<component::Unit>, Without<component::Ghost>)>,
    ) {
    for (mut atlas, mut indices, mut timer, facing, state) in query.iter_mut() {
        let direction = angle_to_direction(facing.value);
        if state.value != State::Move {
            atlas.index = direction * 8;
            continue;
        }
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            indices.current += 1;
            if indices.current > indices.last {
                indices.current = indices.first;
            }
            atlas.index = direction * 8 + indices.current;
        }
    }
}


pub fn draw_gizmo(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Selected>>
    ) {
    for transform in query.iter() {
        gizmos.circle_2d(transform.translation.xy() - Vec2::new(0.0, 25.0), 25.0, Color::GREEN);
    }
}

pub fn gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    ) {
        let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
        config.line_width = 1.0;
}

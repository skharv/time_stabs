use std::{collections::VecDeque, f32::consts::PI};
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
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
                    action::read_action,
                    action::attack,
                    history::track_history,
                    history::repeat_history,
                    history::start_repeat,
                    history::start_reverse,
                    movement::arrive,
                    movement::apply_velocity.after(collision::collision),
                    movement::calculate_direct_velocity.after(movement::turn_towards_target),
                    movement::turn_towards_target,
                    collision::collision.after(movement::calculate_direct_velocity),
                    ));
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
    ) {
    let unit_radius = 10.0;
    for n in 0..10 {
        let mut spawn_transform = Transform::from_xyz((n * 20) as f32, (n * 20) as f32, 0.0);
        spawn_transform.rotate_z((2.0 * PI / 10.0) * n as f32);
        commands.spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: unit_radius })),
            material: materials.add(Color::WHITE),
            transform: spawn_transform,
            ..default()
        },
        Selectable,
        component::Unit,
        component::Radius { value: unit_radius },
        component::Velocity { x: 0.0, y: 0.0 },
        component::MoveSpeed { value: 100.0 },
        component::TurnRate { value: 10.0 },
        component::Target { x: 0.0, y: 0.0 },
        component::CurrentAction { value: action::Action::None },
        component::Attack { timer: Timer::from_seconds(5.0, TimerMode::Once) },
        component::CurrentState { value: State::Idle },
        component::History { snapshots: VecDeque::new() },
        ));
    }
}

pub fn draw_gizmo(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Selected>>
    ) {
    for transform in query.iter() {
        let direction = (transform.rotation * Vec3::Y).truncate().normalize();
        gizmos.circle_2d(transform.translation.xy(), 15.5, Color::GREEN);
        gizmos.line_2d(
            transform.translation.xy(),
            transform.translation.xy() + Vec2::new(direction.x, direction.y) * 25.0,
            Color::BLUE,
        );
    }
}

pub fn gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    ) {
        let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
        config.line_width = 1.0;
}

use std::collections::VecDeque;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::input::component::{Selectable, Selected};

pub mod action;
pub mod component;
mod collision;
mod history;
mod movement;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (
                    draw_gizmo,
                    gizmo_config,
                    action::read_action,
                    history::track_history,
                    history::repeat_history,
                    movement::arrive,
                    movement::direct_movement.after(collision::collision),
                    movement::calculate_direct_velocity,
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
        commands.spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: unit_radius })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz((n * 20) as f32, (n * 20) as f32, 0.0),
            ..default()
        },
        Selectable,
        component::Unit,
        component::Radius { value: unit_radius },
        component::Velocity { x: 0.0, y: 0.0 },
        component::MoveSpeed { value: 100.0 },
        component::TurnRate { value: 0.0 },
        component::Facing { value: 0.0 },
        component::Target { x: 0.0, y: 0.0 },
        component::State { value: action::Action::Idle },
        component::History { snapshots: VecDeque::new() },
        ));
    }
}

pub fn draw_gizmo(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Selected>>
    ) {
    for transform in query.iter() {
        gizmos.circle_2d(transform.translation.xy(), 15.5, Color::GREEN);
    }
}

pub fn gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    ) {
        let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
        config.line_width = 1.0;
}

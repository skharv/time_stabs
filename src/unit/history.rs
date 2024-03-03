use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use super::component;
use crate::input::{component::{Selectable, Selected}, Reverse, Repeat};

#[derive(Clone)]
pub struct Snapshot {
    pub timestamp: f32,
    pub position: Vec2,
}

pub fn track_history(
    mut queue: Query<(&mut component::History, &Transform), (With<component::Unit>, Without<component::Repeat>, Without<component::Reverse>)>,
    time: Res<Time>,
    ) {
    for (mut history, transform) in queue.iter_mut() {
        history.snapshots.push_back(Snapshot {
            timestamp: time.elapsed_seconds(),
            position: transform.translation.xy(),
        });
    }
}

pub fn repeat_history(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queue: Query<(Entity, &Handle<ColorMaterial>, &mut Transform, &mut component::History, Option<&component::Repeat>, Option<&component::Reverse>), (With<component::Unit>, With<component::Ghost>)>,
    ) {
    for (entity, handle, mut transform, mut history, opt_repeat, opt_reverse) in queue.iter_mut() {
        if let Some(_) = opt_repeat {
            if let Some(snapshot) = history.snapshots.pop_front() {
                transform.translation = Vec3::new(snapshot.position.x, snapshot.position.y, 0.0);
            } else {
                if let Some(material) = materials.get_mut(handle) {
                    material.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                }
                commands.entity(entity).remove::<component::Ghost>();
                commands.entity(entity).remove::<component::Repeat>();
                commands.entity(entity).insert(Selectable);
            }
        }
        if let Some(_) = opt_reverse {
            if let Some(snapshot) = history.snapshots.pop_back() {
                transform.translation = Vec3::new(snapshot.position.x, snapshot.position.y, 0.0);
            } else {
                if let Some(material) = materials.get_mut(handle) {
                    material.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                }
                commands.entity(entity).remove::<component::Ghost>();
                commands.entity(entity).remove::<component::Reverse>();
                commands.entity(entity).insert(Selectable);
            }
        }
    }
}

pub fn start_reverse(
    mut commands: Commands,
    mut reverse_reader: EventReader<Reverse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Handle<ColorMaterial>, &mut component::Target, &component::History, &component::Radius, &component::MoveSpeed, &component::State)>,
    time: Res<Time>,
    ) {
    for event in reverse_reader.read() {
        if !event.1 {
            if let Ok((entity, handle, mut target, history, _, _, _)) = query.get_mut(event.0) {
                if let Some(material) = materials.get_mut(handle) {
                    material.color = Color::rgba(0.5, 0.5, 1.0, 0.5);
                }
                if let Some(first_snapshot) = history.snapshots.front() {
                    target.x = first_snapshot.position.x;
                    target.y = first_snapshot.position.y;
                    commands.entity(entity).remove::<Selected>();
                    commands.entity(entity).remove::<Selectable>();
                    commands.entity(entity).insert(component::Ghost);
                    commands.entity(entity).insert(component::Reverse { timestamp: time.elapsed_seconds() });
                }
            }
        } else {
            if let Ok((_, _, _, history, radius, move_speed, state)) = query.get(event.0) {
                if let Some(last_snapshot) = history.snapshots.back() {
                    if let Some(first_snapshot) = history.snapshots.front() {
                        commands.spawn((MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Circle { radius: radius.value })),
                            material: materials.add(Color::rgba(0.5, 0.5, 1.0, 0.5)),
                            transform: Transform::from_xyz(last_snapshot.position.x, last_snapshot.position.y, 0.0),
                            ..default()
                        },
                        component::Ghost,
                        component::Unit,
                        component::Radius { value: radius.value },
                        component::Velocity { x: 0.0, y: 0.0 },
                        component::MoveSpeed { value: move_speed.value },
                        component::TurnRate { value: 0.0 },
                        component::Facing { value: 0.0 },
                        component::Target { x: first_snapshot.position.x, y: first_snapshot.position.y},
                        component::State { value: state.value },
                        component::History { snapshots: history.snapshots.clone() },
                        component::Reverse { timestamp: time.elapsed_seconds() },
                        ));
                    }
                }
            }
        }
    }
}

pub fn start_repeat(
    mut commands: Commands,
    mut repeat_reader: EventReader<Repeat>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Handle<ColorMaterial>, &mut component::Target, &component::History, &component::Radius, &component::MoveSpeed, &component::State)>,
    time: Res<Time>,
    ) {
    for event in repeat_reader.read() {
        if !event.1 {
            if let Ok((entity, handle, mut target, history, _, _, _)) = query.get_mut(event.0) {
                if let Some(material) = materials.get_mut(handle) {
                    material.color = Color::rgba(0.5, 0.5, 1.0, 0.5);
                }
                if let Some(last_snapshot) = history.snapshots.back() {
                target.x = last_snapshot.position.x;
                target.y = last_snapshot.position.y;
                commands.entity(entity).remove::<Selected>();
                commands.entity(entity).remove::<Selectable>();
                commands.entity(entity).insert(component::Ghost);
                commands.entity(entity).insert(component::Repeat{timestamp: time.elapsed_seconds()});
                }
            }
        } else {
            if let Ok((_, _, _, history, radius, move_speed, state)) = query.get(event.0) {
                if let Some(first_snapshot) = history.snapshots.front() {
                    if let Some(last_snapshot) = history.snapshots.back() {
                        commands.spawn((MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Circle { radius: radius.value })),
                            material: materials.add(Color::rgba(0.5, 0.5, 1.0, 0.5)),
                            transform: Transform::from_xyz(first_snapshot.position.x, first_snapshot.position.y, 0.0),
                            ..default()
                        },
                        component::Ghost,
                        component::Unit,
                        component::Radius { value: radius.value },
                        component::Velocity { x: 0.0, y: 0.0 },
                        component::MoveSpeed { value: move_speed.value },
                        component::TurnRate { value: 0.0 },
                        component::Facing { value: 0.0 },
                        component::Target { x: last_snapshot.position.x, y: last_snapshot.position.y},
                        component::State { value: state.value },
                        component::History { snapshots: history.snapshots.clone() },
                        component::Repeat { timestamp: time.elapsed_seconds() },
                        ));
                    }
                }
            }
        }
    }
}

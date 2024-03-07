use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::input::component::{self, AsVec2};
use crate::unit::component::{Radius, Target};
use super::{Deselect, Select, Do};
use crate::unit::State::{Attack, Move};

const BOX_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.25);
const CLICK_ACCURACY: f32 = 2.0;

#[derive(Resource)]
pub struct ControlGroup {
    pub keycode: KeyCode,
    pub entities: Vec<Entity>,
}

pub fn spawn_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::default()));
    commands.spawn((MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(BOX_COLOR),
        visibility: Visibility::Hidden,
        ..default()
    },
    component::Mouse,
    component::ClickPosition { x: 0.0, y: 0.0 },
    ));
}

pub fn show_hide_box(
    mut query: Query<(&mut Visibility, &mut component::ClickPosition, &mut Transform), With<component::Mouse>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (_, mut click_position, mut transform) = query.single_mut();
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                click_position.x = position.x;
                click_position.y = position.y;
                transform.translation = Vec2::new(position.x, position.y).extend(1.0);
            }
        }
    } else if mouse_input.pressed(MouseButton::Left) {
        let (mut visibility, click_position, mut transform) = query.single_mut();
        if *visibility != Visibility::Visible {
            *visibility = Visibility::Visible;
        }
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                let x_difference = position.x - click_position.x;
                let y_difference = position.y - click_position.y;
                let x_midpoint = (click_position.x + position.x) / 2.0;
                let y_midpoint = (click_position.y + position.y) / 2.0;
                transform.translation = Vec2::new(x_midpoint, y_midpoint).extend(1.0);
                transform.scale = Vec2::new(x_difference, y_difference).extend(1.0);
            }
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        let (mut visibility, _, mut transform) = query.single_mut();
        *visibility = Visibility::Hidden;
        transform.scale = Vec2::new(0.0, 0.0).extend(1.0);
    }
}

pub fn select_entities(
    mut commands: Commands,
    mut select_event: EventWriter<Select>,
    mut deselect_event: EventWriter<Deselect>,
    mouse_query: Query<(Entity, &component::ClickPosition, Option<&component::Held>), With<component::Mouse>>,
    selection_query: Query<(Entity, &Transform, &crate::unit::component::Radius, Option<&component::Selected>), With<component::Selectable>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.pressed(MouseButton::Left) && !mouse_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                let (mouse, click_position, opt_held) = mouse_query.single();
                if let Some(_) = opt_held {
                } else {
                    let distance = position.xy().distance(click_position.as_vec2());
                    if distance > CLICK_ACCURACY {
                        commands.entity(mouse).insert(component::Held);
                    }
                }
            }
        }
    }
    if mouse_input.just_released(MouseButton::Left) {
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                let (mouse, click_position, opt_held) = mouse_query.single();
                if let Some(_) = opt_held {
                    for (entity, transform, radius, opt_selected) in selection_query.iter() {
                        let start = Vec2::new(position.x.min(click_position.x), position.y.min(click_position.y));
                        let end = Vec2::new(position.x.max(click_position.x), position.y.max(click_position.y));
                        if let None = opt_selected {
                            if (transform.translation.x >= start.x && transform.translation.x <= end.x) &&
                                (transform.translation.y >= start.y && transform.translation.y <= end.y) {
                                    select_event.send(Select(entity));
                            } else {
                                let distance = transform.translation.xy().distance(position);
                                if distance < radius.value {
                                    select_event.send(Select(entity));
                                }
                            }
                        } else {
                            if !((transform.translation.x >= start.x && transform.translation.x <= end.x) &&
                                (transform.translation.y >= start.y && transform.translation.y <= end.y)) {
                                    deselect_event.send(Deselect(entity));
                            }
                        }
                    }
                    commands.entity(mouse).remove::<component::Held>();
                } else {
                    for (entity, transform, radius, opt_selected) in selection_query.iter() {
                        let distance = transform.translation.xy().distance(position);
                        if let Some(_) = opt_selected {
                            if distance > radius.value {
                                deselect_event.send(Deselect(entity));
                            }
                        } else {
                            if distance <= radius.value {
                                select_event.send(Select(entity));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn act(
    mut do_writer: EventWriter<Do>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut selection_query: Query<(Entity, &mut Target), With<component::Selected>>,
    target_query: Query<(Entity, &Transform, &Radius), Without<component::Selected>>
    ) {
    if mouse_input.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                for (entity, mut target) in selection_query.iter_mut() {
                    let mut target_found = false;
                    for (target_entity, transform, radius) in target_query.iter() {
                        let distance = transform.translation.xy().distance(position);
                        if distance <= radius.value {
                            target_found = true;
                            target.entity = Some(target_entity);
                            target.x = transform.translation.x;
                            target.y = transform.translation.y;
                        }
                    }
                    if target_found {
                        do_writer.send(super::Do(entity, Attack, position.xy()));
                    } else {
                        do_writer.send(super::Do(entity, Move, position.xy()));
                    }
                }
            }
        }
    }
}

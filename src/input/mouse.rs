use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::input::component;
use super::{Select, Deselect};

const BOX_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.25);

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
    component::ClickPosition { x: 0.0, y: 0.0 }
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
    mut select_event: EventWriter<Select>,
    mut deselect_event: EventWriter<Deselect>,
    selection_query: Query<(Entity, &Transform, Option<&component::Selected>), With<component::Selectable>>,
    mouse_query: Query<&component::ClickPosition, With<component::Mouse>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                for (entity, transform, opt_selected) in selection_query.iter() {
                    //This should check to see if the click hits any part of the entity
                    if transform.translation.xy() == position {
                        if let Some(_) = opt_selected {
                            deselect_event.send(Deselect(entity));
                        } else {
                            select_event.send(Select(entity));
                        }
                    }
                }
            }
        }
    } else if mouse_input.pressed(MouseButton::Left) {
        let (camera, camera_transform) = cameras.single();
        if let Some(cursor_position) = windows.single().cursor_position() {
            if let Some(position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                let click_position = mouse_query.single();
                for (entity, transform, opt_selected) in selection_query.iter() {
                    if let None = opt_selected {
                        let start_x = position.x.min(click_position.x);
                        let start_y = position.y.min(click_position.y);
                        let end_x = position.x.max(click_position.x);
                        let end_y = position.y.max(click_position.y);

                        //This should check if any part of the entity is within the box  
                        if transform.translation.x >= start_x && transform.translation.x <= end_x {
                            if transform.translation.y >= start_y && transform.translation.y <= end_y {
                                select_event.send(Select(entity));
                            }
                        }
                    } else {
                        deselect_event.send(Deselect(entity));
                    }
                }
            }
        }
    }
}

use bevy::prelude::*;

use super::component;
use crate::unit::component::{Ghost, Repeat};

const UP: KeyCode = KeyCode::KeyW;
const DOWN: KeyCode = KeyCode::KeyS;
const LEFT: KeyCode = KeyCode::KeyA;
const RIGHT: KeyCode = KeyCode::KeyD;
const SHIFT: KeyCode = KeyCode::ShiftLeft;
const CONTROL: KeyCode = KeyCode::ControlLeft;
const CANCEL: KeyCode = KeyCode::Escape;

pub fn shift_input(
    mut commands: Commands,
    mut reader: EventReader<super::Deselect>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
    for event in reader.read() {
        if !keyboard_input.pressed(SHIFT) {
            commands.entity(event.0).remove::<component::Selected>();
        }
    }
}

pub fn control_input(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Handle<ColorMaterial>), With<component::Selected>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    ) {
    if keyboard_input.just_pressed(CONTROL) {
        for (entity, handle) in query.iter() {
            if let Some(material) = materials.get_mut(handle) {
                material.color = Color::rgba(0.5, 0.5, 1.0, 0.5);
            }
            commands.entity(entity).remove::<component::Selected>();
            commands.entity(entity).remove::<component::Selectable>();
            commands.entity(entity).insert(Ghost);
            commands.entity(entity).insert(Repeat{timestamp: time.elapsed_seconds()});
        }
    }
}


use bevy::prelude::*;

use super::component;

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


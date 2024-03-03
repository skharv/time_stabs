use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use super::{component::{Selected, Selectable}, Reverse, Repeat};
use crate::unit::component;

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
            commands.entity(event.0).remove::<Selected>();
        }
    }
}

pub fn control_input(
    mut reverse_writer: EventWriter<Reverse>,
    mut repeat_writer: EventWriter<Repeat>,
    query: Query<Entity, (With<component::Unit>, With<Selected>, With<component::History>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
    if keyboard_input.just_pressed(CONTROL) {
        if !keyboard_input.pressed(SHIFT){
            for entity in query.iter() {
                repeat_writer.send(Repeat(entity, false));
            }
        } else {
            for entity in query.iter() {
                repeat_writer.send(Repeat(entity, true));
            }
        }
    }
    if keyboard_input.just_pressed(CANCEL) {
        if !keyboard_input.pressed(SHIFT){
            for entity in query.iter() {
                reverse_writer.send(Reverse(entity, false));
            }
        } else {
            for entity in query.iter() {
                reverse_writer.send(Reverse(entity, true));
            }
        }
    }
}


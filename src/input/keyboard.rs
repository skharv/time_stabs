use bevy::{input::{keyboard::KeyboardInput, ButtonState}, prelude::*};

use super::{component::{Selectable, Selected}, ControlGroups, Repeat, Reverse};
use crate::unit::State;
use crate::unit::component;

const UP: KeyCode = KeyCode::KeyW;
const DOWN: KeyCode = KeyCode::KeyS;
const LEFT: KeyCode = KeyCode::KeyA;
const RIGHT: KeyCode = KeyCode::KeyD;
const ATTACK: KeyCode = KeyCode::KeyE;
const STOP: KeyCode = KeyCode::KeyQ;
const SHIFT: KeyCode = KeyCode::ShiftLeft;
const CONTROL: KeyCode = KeyCode::ControlLeft;
const CANCEL: KeyCode = KeyCode::Escape;

pub fn camera_movement(
    mut query: Query<(&Camera, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    ) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if keyboard_input.pressed(UP) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(DOWN) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(LEFT) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(RIGHT) {
            direction.x += 1.0;
        }
        transform.translation += direction * 1000.0 * time.delta_seconds();
    }
}

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
    if keyboard_input.just_pressed(UP) {
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

pub fn double_tap_timer(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut timer: ResMut<super::DoubleTap>,
    time: Res<Time>,
    ) {
    if timer.timer.finished() {
        for event in keyboard_event.read() {
            if event.state == ButtonState::Pressed {
                timer.key = Some(event.key_code);
                timer.timer.reset();
            }
        }
    } else {
       timer.timer.tick(time.delta());
    }
}

pub fn shoot(
    mut do_writer: EventWriter<super::Do>,
    mut query: Query<(Entity, &Transform), (With<component::Unit>, With<Selected>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
    if keyboard_input.just_pressed(ATTACK) {
        for (entity, _) in query.iter_mut() {
            do_writer.send(super::Do(entity, State::Attack, Vec2::new(0.0, 0.0)));
        }
    }
}

pub fn stop(
    mut do_writer: EventWriter<super::Do>,
    mut query: Query<(Entity, &Transform), (With<component::Unit>, With<Selected>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
    if keyboard_input.just_pressed(STOP) {
        for (entity, _) in query.iter_mut() {
            do_writer.send(super::Do(entity, State::Idle, Vec2::new(0.0, 0.0)));
        }
    }
}

pub fn set_control_group(
    query: Query<Entity, (With<component::Unit>, With<Selected>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut control_group: ResMut<ControlGroups>,
    ) {
    if keyboard_input.pressed(CONTROL) {
        let mut keycode: Option<KeyCode> = None;
        if keyboard_input.just_pressed(KeyCode::Digit0) {
            keycode = Some(KeyCode::Digit0);
        }
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            keycode = Some(KeyCode::Digit1);
        }
        if keyboard_input.just_pressed(KeyCode::Digit2) {
            keycode = Some(KeyCode::Digit2);
        }
        if keyboard_input.just_pressed(KeyCode::Digit3) {
            keycode = Some(KeyCode::Digit3);
        }
        if keyboard_input.just_pressed(KeyCode::Digit4) {
            keycode = Some(KeyCode::Digit4);
        }
        if keyboard_input.just_pressed(KeyCode::Digit5) {
            keycode = Some(KeyCode::Digit5);
        }
        if keyboard_input.just_pressed(KeyCode::Digit6) {
            keycode = Some(KeyCode::Digit6);
        }
        if keyboard_input.just_pressed(KeyCode::Digit7) {
            keycode = Some(KeyCode::Digit7);
        }
        if keyboard_input.just_pressed(KeyCode::Digit8) {
            keycode = Some(KeyCode::Digit8);
        }
        if keyboard_input.just_pressed(KeyCode::Digit9) {
            keycode = Some(KeyCode::Digit9);
        }
        if let Some(key) = keycode {
            let mut entities = Vec::new();
            query.iter().for_each(|entity| {
                entities.push(entity);
            });
            control_group.groups.entry(key).or_insert(entities);
        }
    }
}

pub fn get_control_group(
    mut commands: Commands,
    timer: Res<super::DoubleTap>,
    query: Query<(Entity, &Transform), (With<component::Unit>, With<Selectable>, Without<Camera>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    control_group: Res<ControlGroups>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<component::Unit>)>, 
    ) {
    let mut keycode: Option<KeyCode> = None;
    if keyboard_input.just_pressed(KeyCode::Digit0) {
        keycode = Some(KeyCode::Digit0);
    }
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        keycode = Some(KeyCode::Digit1);
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        keycode = Some(KeyCode::Digit2);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        keycode = Some(KeyCode::Digit3);
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        keycode = Some(KeyCode::Digit4);
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        keycode = Some(KeyCode::Digit5);
    }
    if keyboard_input.just_pressed(KeyCode::Digit6) {
        keycode = Some(KeyCode::Digit6);
    }
    if keyboard_input.just_pressed(KeyCode::Digit7) {
        keycode = Some(KeyCode::Digit7);
    }
    if keyboard_input.just_pressed(KeyCode::Digit8) {
        keycode = Some(KeyCode::Digit8);
    }
    if keyboard_input.just_pressed(KeyCode::Digit9) {
        keycode = Some(KeyCode::Digit9);
    }
    let mut selection_count = 0;
    let mut position = Vec2::new(0.0, 0.0);
    if let Some(key) = keycode {
        if let Some(entities) = control_group.groups.get(&key) {
            for (entity, transform) in query.iter() {
                if let Some(found) = entities.iter().find(|&&x| x == entity) {
                    selection_count += 1;
                    position += transform.translation.truncate();
                    commands.entity(*found).insert(Selected);
                } else {
                    commands.entity(entity).remove::<Selected>();
                } 
            }
        }
    }
    if keycode == timer.key && selection_count > 0 {
    if !timer.timer.finished() {
            position /= selection_count as f32;
            let mut camera_transform = cameras.single_mut();
            camera_transform.translation = Vec3::new(position.x, position.y, camera_transform.translation.z);
        }
    }
}

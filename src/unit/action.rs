use std::f32::consts::PI;

use bevy::prelude::*;

use crate::input::Do;
use super::State;
use super::component;
use crate::input::component::Selected;
use crate::bullet::Fire;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    None,
    Attack,
    Target,
    Stop,
    Halt,
}

pub fn read_action(
    mut do_event: EventReader<Do>,
    mut query: Query<(&mut component::Target, &mut component::CurrentState, &mut component::CurrentAction), (With<component::Unit>, With<Selected>)>,
    ) {
    for event in do_event.read() {
        match event.0 {
            State::Move => {
                for (mut target, mut state, mut action) in query.iter_mut() {
                    target.x = event.1.x;
                    target.y = event.1.y;
                    state.value = State::Move;
                    action.value = Action::None;
                }
            },
            State::Attack => {
                for (mut target, mut state, mut action) in query.iter_mut() {
                    target.x = event.1.x;
                    target.y = event.1.y;
                    state.value = State::Attack;
                    action.value = Action::Attack;
                }
            },
            _ => {}
        }
    }
}

pub fn attack(
    mut fire_writer: EventWriter<Fire>,
    mut query: Query<(&mut component::CurrentAction, &component::CurrentState, &mut component::Attack, &Transform, &component::Facing), With<component::Unit>>,
    time: Res<Time>,
    ) {
    for (mut action, state, mut attack, transform, facing) in query.iter_mut() {
        if action.value == Action::Attack {
            fire_writer.send(Fire(Vec2::new(transform.translation.x, transform.translation.y), facing.value - (PI / 2.0)));
            action.value = Action::None;
            attack.timer.reset();
        }
        if state.value == State::Attack {
            attack.timer.tick(time.delta());
            if attack.timer.finished() {
                action.value = Action::Attack;
            } else {
                action.value = Action::None;
            }
        }
    }
}


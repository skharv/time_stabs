use std::f32::consts::PI;

use bevy::prelude::*;

use crate::input::Do;
use super::State;
use super::{component, component::AsVec2};
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
    mut query: Query<(&mut component::Target, &mut component::CurrentState, &mut component::CurrentAction), With<component::Unit>>,
    ) {
    for event in do_event.read() {
        match event.1 {
            State::Move => {
                if let Ok((mut target, mut state, mut action)) = query.get_mut(event.0) {
                    target.x = event.2.x;
                    target.y = event.2.y;
                    state.value = State::Move;
                    action.value = Action::None;
                }
            },
            State::Attack => {
                if let Ok((mut target, mut state, mut action)) = query.get_mut(event.0) {
                    target.x = event.2.x;
                    target.y = event.2.y;
                    state.value = State::Attack;
                    action.value = Action::Attack;
                }
            },
            State::Idle => {
                if let Ok((mut target, mut state, mut action)) = query.get_mut(event.0) {
                    target.x = event.2.x;
                    target.y = event.2.y;
                    target.entity = None;
                    state.value = State::Idle;
                    action.value = Action::None;
                }
            },
            _ => {}
        }
    }
}

pub fn attack(
    mut fire_writer: EventWriter<Fire>,
    mut query: Query<(&mut component::CurrentAction, &component::CurrentState, &mut component::Attack, &Transform, &component::Facing, &component::Target), With<component::Unit>>,
    time: Res<Time>,
    ) {
    for (mut action, state, mut attack, transform, facing, target) in query.iter_mut() {
        if action.value == Action::Attack {
            let forward = Vec2::new(facing.value.cos(), facing.value.sin()).normalize();
            let to_target = (target.as_vec2() - transform.translation.xy()).normalize();
            let forward_dot_target = forward.dot(to_target);
            if (forward_dot_target - 1.0).abs() < f32::EPSILON {
                fire_writer.send(Fire(transform.translation.xy() + (to_target * 50.0), facing.value - (PI / 2.0)));
                action.value = Action::None;
                attack.timer.reset();
            }
        }
        if state.value == State::Attack {
            attack.timer.tick(time.delta());
            if attack.timer.finished() {
                action.value = Action::Attack;
            }
        }
    }
}

pub fn engage(
    mut do_writer: EventWriter<Do>,
    mut query: Query<(Entity, &Transform, &mut component::CurrentState, &component::Unit, &mut component::Target, &component::Attack)>,
    ) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut unit1, mut unit2]) = combinations.fetch_next() {
        if unit1.3.owner == unit2.3.owner {
            continue;
        }
        if unit1.2.value == State::AttackMove || unit1.2.value == State::Idle {
            if unit1.1.translation.xy().distance(unit2.1.translation.xy()) <= unit1.5.range {
                unit1.4.entity = Some(unit2.0);           
                do_writer.send(Do(unit1.0, State::Attack, unit2.1.translation.xy()));
            }
        }
        if unit2.2.value == State::AttackMove || unit2.2.value == State::Idle {
            if unit2.1.translation.xy().distance(unit1.1.translation.xy()) <= unit2.5.range {
                unit2.4.entity = Some(unit1.0);
                do_writer.send(Do(unit2.0, State::Attack, unit1.1.translation.xy()));
            }
        }
    }
}


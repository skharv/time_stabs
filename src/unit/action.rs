use bevy::prelude::*;

use crate::input::Do;
use super::component;
use crate::input::component::Selected;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Idle,
    Move,
    Attack,
    Stop,
    Halt,
}

pub fn read_action(
    mut do_event: EventReader<Do>,
    mut query: Query<(&mut component::Target, &mut component::State), (With<component::Unit>, With<Selected>)>,
    ) {
    for event in do_event.read() {
        match event.0 {
            Action::Move => {
                for (mut target, mut state) in query.iter_mut() {
                    target.x = event.1.x;
                    target.y = event.1.y;
                    state.value = Action::Move;
                }
            },
            _ => {}
        }
    }
}

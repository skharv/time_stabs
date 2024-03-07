use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::prelude::State;

use crate::unit;
use crate::AppState;

pub mod component;
mod keyboard;
mod mouse;

#[derive(Event)]
pub struct Select(Entity);

#[derive(Event)]
pub struct Deselect(Entity);

#[derive(Event)]
pub struct Do(pub Entity, pub unit::State, pub Vec2);

#[derive(Event)]
pub struct Queue(pub Entity, pub unit::State, pub Vec2);

#[derive(Event)]
pub struct Repeat(pub Entity, pub bool);

#[derive(Event)]
pub struct Reverse(pub Entity, pub bool);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mouse::spawn_box)
            .add_systems(Update, (
                    mouse::show_hide_box,
                    mouse::select_entities,
                    mouse::act,
                    keyboard::shoot,
                    keyboard::stop,
                    keyboard::shift_input,
                    keyboard::control_input,
                    selection,
                    ))
            .add_systems(Update, next_state)
            .add_event::<Select>()
            .add_event::<Deselect>()
            .add_event::<Do>()
            .add_event::<Repeat>()
            .add_event::<Reverse>();
    }
}

pub fn selection(
    mut commands: Commands,
    mut event_reader: EventReader<Select>,
    ) {
    for event in event_reader.read() {
        commands.entity(event.0).insert(component::Selected);
    }
}

pub fn next_state(
    mut keyboard_event: EventReader<KeyboardInput>,
    mut mouse_event: EventReader<MouseButtonInput>,
    current_state: Res<State<AppState>>,
    mut app_state: ResMut<NextState<AppState>>,
    ) {
    let mut input = false;
    for kb_event in keyboard_event.read() {
        match kb_event.state {
            ButtonState::Released => {
                input = true;
            }
            _ => {
                return;
            }
        }
    }
    for m_event in mouse_event.read() {
        match m_event.state {
            ButtonState::Released => {
                input = true;
            }
            _ => {
                return;
            }
        }
    }
    if input {
        match current_state.get() {
            AppState::Start => {
                info!("Start -> RoundStart");
                app_state.set(AppState::RoundStart)
            },
            AppState::RoundStart => {
                info!("RoundStart -> InGame");
                app_state.set(AppState::InGame)
            },
            AppState::RoundEnd => {
                info!("RoundEnd -> RoundStart");
                app_state.set(AppState::RoundStart)
            },
            _ => {}
        }
    }
}

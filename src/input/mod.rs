use bevy::prelude::*;

use crate::unit::action::Action;

pub mod component;
mod keyboard;
mod mouse;

#[derive(Event)]
pub struct Select(Entity);

#[derive(Event)]
pub struct Deselect(Entity);

#[derive(Event)]
pub struct Do(pub Action, pub Vec2);

#[derive(Event)]
pub struct Queue(pub Action, pub Vec2);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mouse::spawn_box)
            .add_systems(Update, (
                    mouse::show_hide_box,
                    mouse::select_entities,
                    mouse::act,
                    keyboard::shift_input,
                    keyboard::control_input,
                    selection,
                    ))
            .add_event::<Select>()
            .add_event::<Deselect>()
            .add_event::<Do>();
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

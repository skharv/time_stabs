use bevy::prelude::*;

pub mod component;
mod keyboard;
mod mouse;

#[derive(Event)]
pub struct Select(Entity);

#[derive(Event)]
pub struct Deselect(Entity);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mouse::spawn_box)
            .add_systems(Update, (
                    mouse::show_hide_box,
                    mouse::select_entities,
                    mouse::click_tick,
                    keyboard::shift_input,
                    selection,
                    ))
            .add_event::<Select>()
            .add_event::<Deselect>();
    }
}

pub fn selection(
    mut commands: Commands,
    mut event_reader: EventReader<Select>,
    ) {
    for event in event_reader.read() {
        info!("selecting {:?} from event!", event.0);
        commands.entity(event.0).insert(component::Selected);
    }
}

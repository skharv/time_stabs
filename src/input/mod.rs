use bevy::prelude::*;

mod mouse;
mod component;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mouse::spawn_box)
            .add_systems(Update, mouse::show_hide_box);
    }
}

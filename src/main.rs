use bevy::prelude::*;

mod input;
mod unit;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        (DefaultPlugins,
         input::InputPlugin,
         unit::UnitPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

use bevy::prelude::*;

mod input;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        (DefaultPlugins,
         input::InputPlugin
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

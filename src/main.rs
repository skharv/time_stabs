use bevy::prelude::*;

mod bullet;
mod game;
mod input;
mod ui;
mod unit;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    Start,
    RoundStart,
    RoundEnd,
    InGame,
    Pause,
    Win,
    Loss,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        (DefaultPlugins,
         input::InputPlugin,
         unit::UnitPlugin,
         bullet::BulletPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .init_state::<AppState>()
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

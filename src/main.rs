use bevy::prelude::*;

mod bullet;
mod camera;
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
         camera::CameraPlugin,
        ))
        .init_state::<AppState>()
        .run();
}


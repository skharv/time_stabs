use bevy::prelude::*;
use crate::AppState;

mod component;

#[derive(Resource)]
pub struct Round {
    pub timer: Timer,
    pub attempts: u32,
}

const ROUND_DURATION: f32 = 15.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Round {
            timer: Timer::from_seconds(ROUND_DURATION, TimerMode::Once),
            attempts: 0,
        })
        .add_systems(OnEnter(AppState::RoundStart), start_round)
        .add_systems(Update, count_round_time.run_if(in_state(AppState::InGame)))
        .add_systems(Update, end_round.run_if(in_state(AppState::InGame)));
    }
}

fn start_round (
    mut round: ResMut<Round>,
    ) {
    round.timer.reset();
    round.attempts += 1;
}

fn count_round_time (
    time: Res<Time>,
    mut round: ResMut<Round>,
    ) {
    round.timer.tick(time.delta());
    info!("Round {} - Time left: {}", round.attempts, round.timer.remaining().as_secs_f32());
}

fn end_round (
    mut app_state: ResMut<NextState<AppState>>,
    round: Res<Round>,
    ) {
    if round.timer.finished() {
        app_state.set(AppState::RoundEnd);
    }
}

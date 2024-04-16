use bevy::prelude::*;
use crate::game;
use game::{WaveState, GameState};
use crate::AppState;

pub mod layout;

pub struct WaveStartPlugin;

#[derive(Resource)]
struct WaveCountdownTimer(Timer);

#[derive(Resource)]
struct CountdownNum(u8);

impl Plugin for WaveStartPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WaveCountdownTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(OnEnter(WaveState::Start), (layout::spawn_wave_start, initialise_countdown_num))
            .add_systems(OnExit(WaveState::Start), layout::despawn_wave_start)
            .add_systems(Update, update_countdown_text.run_if(in_state(AppState::Game)).run_if(in_state(WaveState::Start)).run_if(in_state(GameState::Running)));
    }
}

// Initialize starting countdown num
fn initialise_countdown_num(mut commands: Commands) {
    commands.insert_resource(
        CountdownNum(game::WAVE_COUNTDOWN_SECONDS)
    );
}

// Update countdown text and change WaveState once the countdown as finished
fn update_countdown_text(
    mut wave_countdown_timer: ResMut<WaveCountdownTimer>,
    mut countdown_num: ResMut<CountdownNum>,
    mut text_query: Query<&mut Text, With<layout::CountdownText>>,
    time: Res<Time>,
    mut next_wave_state: ResMut<NextState<WaveState>>,
) {
    wave_countdown_timer.0.tick(time.delta());
    if wave_countdown_timer.0.just_finished() {
        if let Ok(mut text) = text_query.get_single_mut() {

            // Once the countdown has reached 0 transition to WaveState::Fight
            if countdown_num.0 == 0 {
                next_wave_state.set(WaveState::Fight);
                return;
            }

            countdown_num.0 -= 1;
            text.sections[0].value = format!("{}", countdown_num.0);
        }
    }
}
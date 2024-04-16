use bevy::prelude::*;

use crate::{AppState, game::GameState};

pub mod layout;
pub mod interactions;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), layout::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), layout::despawn_pause_menu)
            .add_systems(Update, (change_pause_state, interactions::resume_button_interactions).run_if(in_state(AppState::Game)));
    }
}

// Toggle pause menu when the escape button is pressed
fn change_pause_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {


        let state = match game_state.get() {
            GameState::Paused => GameState::Running,
            GameState::Running => GameState::Paused,
            GameState::GameOver => GameState::GameOver,
        };

        next_state.set(state);
    }

    // Prevent cases where pause menu and game over menu are open at once
    // When pause menu is opened on the same frame as the player death
    if app_state.get() != &AppState::Game {
        next_state.set(GameState::Running);
    }
}
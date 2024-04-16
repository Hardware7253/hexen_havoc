use bevy::prelude::*;
use crate::AppState;
use super::layout;

// Start the game when the play button is pressed
pub fn play_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<layout::PlayButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        if interaction == &Interaction::Pressed {
            next_state.set(AppState::GameSetup)
        }
    }
}

// Send exit app event when the quit button is pressed
pub fn quit_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<layout::QuitButton>)>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        if interaction == &Interaction::Pressed {
            app_exit_events.send(bevy::app::AppExit);
        }
    }
}
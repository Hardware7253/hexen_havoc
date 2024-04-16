use bevy::prelude::*;

use crate::{AppState, GameCleanupEvent};
use super::layout;

// Restart the game
pub fn restart_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<layout::RestartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut cleanup_event: EventWriter<GameCleanupEvent>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match interaction {
            Interaction::Pressed => {
                cleanup_event.send(GameCleanupEvent{next_state: AppState::GameSetup}); // Cleanup the game and then setup the game to restart
                next_state.set(AppState::GameCleanup);
            },
            _ => (), 
        }
    }
}
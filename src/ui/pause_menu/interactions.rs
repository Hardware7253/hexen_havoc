use bevy::prelude::*;

use crate::game::GameState;
use super::layout;

// Unpause the game
pub fn resume_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<layout::ResumeButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        if interaction == &Interaction::Pressed {
            next_state.set(GameState::Running);
        }
    }
}
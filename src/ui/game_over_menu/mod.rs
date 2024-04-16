use bevy::prelude::*;

use crate::game::GameState;

pub mod layout;
pub mod interactions;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), layout::spawn_game_over_menu)
            .add_systems(OnExit(GameState::GameOver), layout::despawn_game_over_menu)
            .add_systems(Update, interactions::restart_button_interactions.run_if(in_state(GameState::GameOver)));
    }
}


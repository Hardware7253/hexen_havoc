use bevy::prelude::*;
use crate::AppState;

pub mod layout;
pub mod interactions;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), layout::spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), layout::despawn_main_menu)
            .add_systems(Update, (interactions::play_button_interactions, interactions::quit_button_interactions).run_if(in_state(AppState::MainMenu)));
    }
}
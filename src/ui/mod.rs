use bevy::prelude::*;

pub mod styles;
pub mod helpers;

pub mod main_menu;
pub mod hud;
pub mod pause_menu;
pub mod wave_start;
pub mod game_over_menu;

use crate::{GameCleanupEvent, AppState};

#[derive(Component)]
pub struct GenericButton;

#[derive(Component)]
pub struct GenericBackButton;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((main_menu::MainMenuPlugin, hud::HudPlugin, pause_menu::PauseMenuPlugin, wave_start::WaveStartPlugin, game_over_menu::GameOverMenuPlugin))
        .add_systems(Update, (button_interactions, back_button_interactions));
    }
}

// Changes button colors when the user interacts with it
pub fn button_interactions(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => *background_color = Color::hex(styles::BUTTON_PRESSED_HEX).unwrap().into(),
            Interaction::Hovered => *background_color = Color::hex(styles::BUTTON_HOVER_HEX).unwrap().into(),
            Interaction::None => *background_color = Color::hex(styles::BUTTON_DEFAULT_HEX).unwrap().into(),
        }
    }
}

// Go back the main menu
// This code is here to avoid duplicates, since there is a back button in the pause and game over menu
pub fn back_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<GenericBackButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut cleanup_event: EventWriter<GameCleanupEvent>,
) {
    if let Ok(interaction) = button_query.get_single() {
        if interaction == &Interaction::Pressed {
            cleanup_event.send(GameCleanupEvent{next_state: AppState::MainMenu}); // Cleanup the game and then transition to MainMenu
            next_app_state.set(AppState::GameCleanup);
        }
    }
}
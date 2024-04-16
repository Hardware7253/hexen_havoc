use bevy::prelude::*;
use crate::AppState;
use crate::game::{player, GameState};

pub mod layout;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), layout::spawn_hud)
            .add_systems(OnExit(AppState::Game), layout::despawn_hud)
            .add_systems(Update, update_health_text.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)));
    }
}

fn update_health_text(
    mut text_query: Query<&mut Text, With<layout::HealthText>>,
    player_query: Query<&player::Player, Changed<player::Player>>
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Ok(player) = player_query.get_single() {
            text.sections[0].value = format!("{}", player.health);
        }
    }
}
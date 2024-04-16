use bevy::prelude::*;
use crate::ui::{styles, helpers};
use crate::game;

#[derive(Component)]
pub struct WaveStart;

#[derive(Component)]
pub struct CountdownText;

pub fn spawn_wave_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text = format!("{}", game::WAVE_COUNTDOWN_SECONDS);
    commands.spawn(
        
        (
            WaveStart,

            // Create a nodebundle accross the top of the screen to contain the countdown text
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),

                    ..default()
                },
                ..default()
            }
        )
    )
    
    .with_children(|parent| {
        // Countdown text
        parent.spawn(
            (
                CountdownText,
                helpers::text(&text, styles::TITLE_TEXT_STYLE, &asset_server)
            )
        );
    });
}

pub fn despawn_wave_start(mut commands: Commands, hud_query: Query<Entity, With<WaveStart>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
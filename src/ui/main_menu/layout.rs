use bevy::prelude::*;
use crate::ui;
use ui::{styles, helpers};

#[derive(Component)]
pub struct MainMenu;

// Components for individual buttons
#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(

        // Main menu background, and parent node
        (
            MainMenu,

            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    row_gap: Val::Percent(1.0),
                    ..default()
                },
                background_color: Color::hex(styles::BACKGROUND_HEX).unwrap().into(),
                ..default()
            }
        )
    )
    
    .with_children(|parent| {

        // Title
        parent.spawn(
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }
        ).with_children(|parent| {
            parent.spawn(
                helpers::text("Hexen Havoc", styles::TITLE_TEXT_STYLE, &asset_server)
            );
        });
        
        // Play button
        parent.spawn(
            (
                ui::GenericButton,
                PlayButton,
                helpers::button(styles::BUTTON_STYLE),
            )   
        )
        .with_children(|parent| {
            parent.spawn(
                helpers::text("Play", styles::BUTTON_STYLE.text_style.unwrap(), &asset_server) // Play button text
            );
        });

        // Quit button
        parent.spawn(
            (
                ui::GenericButton,
                QuitButton,
                helpers::button(styles::BUTTON_STYLE),
            )   
        )
        .with_children(|parent| {
            parent.spawn(
                helpers::text("Quit", styles::BUTTON_STYLE.text_style.unwrap(), &asset_server) // Play button text
            );
        });

    });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
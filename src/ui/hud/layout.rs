use bevy::prelude::*;
use crate::art;
use crate::ui::{styles, helpers};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct HealthText;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        
        (
            Hud,

            // Create a nodebundle accross the top of the screen to contain the hud
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    
                    width: Val::Percent(100.0),
                    height: Val::Percent(5.0),

                    ..default()
                },
                ..default()
            }
        )
    )
    
    .with_children(|parent| {

        // Health image
        parent.spawn(
            NodeBundle { // Create a nodebundle to center the image
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    
    
                    width: Val::Percent(3.0),
                    height: Val::Percent(100.0),

                    ..default()
                },
                ..default()
            }
        ).with_children(|parent| {
            parent.spawn(
                ImageBundle {
                    image: UiImage::new(asset_server.load(art::HEALTH_SPRITE_PATH)),
                    transform: Transform::from_scale(Vec3::splat(1.0)),
                    ..default()
                }
            );
        });

        // Health text
        parent.spawn(
            (
                HealthText,
                helpers::text("0", styles::BODY_TEXT_STYLE, &asset_server)
            )
        );
    });
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
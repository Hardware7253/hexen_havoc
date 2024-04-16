use bevy::prelude::*;
use crate::{AppState, art, game};
use bevy::window::PrimaryWindow;
use game::{GameState, helpers, projectiles};

#[derive(Component)]
pub struct Projectile {
    pub direction_vector: Vec3,
    pub projectile_types_index: usize, // Index of bullet in ProjectileTypes resource
}

pub struct ProjectileInfo {
    pub damage: i32,
    pub speed: f32,
    pub piercing: bool,
    pub enemy: bool,
    pub sprite_information: art::SpriteInformation,
}

#[derive(Resource)]
pub struct ProjectileTypes(pub Vec<ProjectileInfo>);

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ProjectileTypes(
            vec![
                // Enemy projectiles
                ProjectileInfo {
                    damage: 2,
                    speed: 250.0,
                    piercing: false,
                    enemy: true,
                    sprite_information: art::ENEMY_PROJECTILE_SPRITE,
                },
                
                ProjectileInfo {
                    damage: 3,
                    speed: 280.0,
                    piercing: false,
                    enemy: true,
                    sprite_information: art::ENEMY_PROJECTILE_SPRITE,
                },

                ProjectileInfo {
                    damage: 4,
                    speed: 300.0,
                    piercing: false,
                    enemy: true,
                    sprite_information: art::ENEMY_PROJECTILE_SPRITE,
                },

                ProjectileInfo {
                    damage: 8,
                    speed: 330.0,
                    piercing: false,
                    enemy: true,
                    sprite_information: art::ENEMY_PROJECTILE_SPRITE,
                },

                // Friendly projectiles
                ProjectileInfo {
                    damage: 1,
                    speed: 350.0,
                    piercing: false,
                    enemy: false,
                    sprite_information: art::FRIENDLY_PROJECTILE_1_SPRITE,
                },

                ProjectileInfo {
                    damage: 2,
                    speed: 400.0,
                    piercing: true,
                    enemy: false,
                    sprite_information: art::FRIENDLY_PROJECTILE_2_SPRITE,
                },
            ]
        ))
        .add_systems(Update, move_projectiles.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)));
        
    }
}

// Moves all projectiles, and despawns them when they go off screen
fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(&mut Transform, &projectiles::Projectile, Entity)>,
    projectile_types: Res<projectiles::ProjectileTypes>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, projectile, entity) in projectile_query.iter_mut() {
        let projectile_info = &projectile_types.0[projectile.projectile_types_index];
        transform.translation += projectile.direction_vector * projectile_info.speed * time.delta_seconds();

        if !helpers::is_on_screen(&transform.translation, &window) {
            commands.entity(entity).despawn();
        }
    }
}
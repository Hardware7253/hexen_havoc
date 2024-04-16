use bevy::prelude::*;
use std::time::Instant;
use crate::{AppState, art, game};
use bevy::window::PrimaryWindow;
use game::{player, WaveState, GameState, helpers, projectiles, GameComponent, characters};

mod spawner;

#[derive(Component)]
pub struct Enemy;

pub const ENEMY_RADIUS: f32 = 32.0 * art::SPRITE_SCALE; // Radius which the enemy wants to keep clear of other enemies

#[derive(Resource, Clone, Debug)]
pub struct EnemyPositions(pub Vec<Option<Vec3>>);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameSetup), insert_enemy_positions)
            .add_systems(OnEnter(WaveState::Fight), spawner::spawn_enemies)
            .add_systems(Update, (move_enemies, shoot).run_if(in_state(AppState::Game)).run_if(in_state(WaveState::Fight)).run_if(in_state(GameState::Running)));
    }
}

fn insert_enemy_positions(mut commands: Commands) {
    commands.insert_resource(EnemyPositions(Vec::new()));
}

// Moves all enemies towards the player
pub fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &mut characters::Character), With<Enemy>>,
    player_query: Query<&Transform, (With<player::Player>, Without<Enemy>)>,
    enemy_types: Res<characters::EnemyTypes>,
    time: Res<Time>,
    mut enemy_positions: ResMut<EnemyPositions>,
) {
    let player_translation = player_query.get_single().unwrap().translation;

    for (mut enemy_transform, mut enemy) in enemy_query.iter_mut() {

        // Avoid other enemies
        helpers::avoid_positions(&mut enemy_transform, &enemy_positions.0, enemy.position_index, ENEMY_RADIUS, &time);

        // Direction vector always points towards the player
        let direction_vector = helpers::vector_subtract(&player_translation, &enemy_transform.translation);
        let direction_vector = direction_vector.normalize();

        enemy.direction_vector = direction_vector;

        // Move enemy
        enemy_transform.translation += enemy_types.0[enemy.type_index].speed * direction_vector * time.delta_seconds();

        // Update enemy position
        if enemy_positions.0[enemy.position_index] != None {
            enemy_positions.0[enemy.position_index] = Some(enemy_transform.translation);
        }
    }
}

// Make the enemies shoot at the player
fn shoot(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut characters::Character), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_types: Res<characters::EnemyTypes>,
    projectile_types: Res<projectiles::ProjectileTypes>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let enemy_translation = enemy_transform.translation;

        // Enemies only shoot when they are on screen
        if helpers::is_on_screen(&enemy_translation, window) {

            match enemy.last_shot {
                Some(instant) => {


                    if instant.elapsed().as_secs_f32() > enemy_types.0[enemy.type_index].firing_rate.unwrap() {
                        enemy.last_shot = Some(Instant::now());
                        let projectile_types_index = enemy_types.0[enemy.type_index].projectile_types_index.unwrap();

                        commands.spawn(
                            (
                                projectiles::Projectile {
                                    direction_vector: enemy.direction_vector,
                                    projectile_types_index: projectile_types_index,
                                },
                                GameComponent,
                                SpriteBundle {
                                    transform: Transform {
                                        translation: enemy_translation,
                                        scale: Vec3::splat(art::SPRITE_SCALE),
                                        ..default()
                                    },
                                    texture: asset_server.load(projectile_types.0[projectile_types_index].sprite_information.sprite_path),
                                    ..default()
                                }
                            )
                        );
                    }
                },
                None => enemy.last_shot = Some(Instant::now()),
            }
        }
    }
}


use bevy::prelude::*;
use rand::Rng;
use bevy::window::PrimaryWindow;
use std::time::{Instant, Duration};

use crate::{game, AppState, art};
use game::{WaveState, GameState, characters, helpers, enemies, projectiles, player, NANOS_PER_MILLIS, animation};

#[derive(Component)]
pub struct Summon;

pub const SUMMON_RADIUS: f32 = 16.0 * art::SPRITE_SCALE; // Radius which summons want to keep clear of other summons

#[derive(Resource, Clone)]
pub struct SummonPositions(pub Vec<Option<Vec3>>);

pub struct SummonPlugin;

impl Plugin for SummonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameSetup), insert_summon_positions)
            .add_systems(Update, (move_summons, spawn_summons).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))
            .add_systems(Update, shoot_ranged_attacks.run_if(in_state(AppState::Game)).run_if(in_state(WaveState::Fight)).run_if(in_state(GameState::Running)));
    }
}

fn insert_summon_positions(mut commands: Commands) {
    commands.insert_resource(SummonPositions(Vec::new()));
}


// Spawn summons based of player collectibles
fn spawn_summons(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    summon_types: Res<characters::SummonTypes>,
    mut summon_positions: ResMut<SummonPositions>,
    mut player_query: Query<&mut player::Player>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    if let Some(mut player) = player_query.iter_mut().next() {
        
        // Iterate through collectibles
        for (i, collectibles_amount) in player.collectibles.iter_mut().enumerate() {

            // Check summons against this type of collectible to see if they can be spawned
            for (j, summon_info) in summon_types.0.iter().enumerate() {
                if let Some(ammount_required) = summon_info.collectible_ammount_required {

                    // Check the player has enough of the correct type of collectibles to spawn the summon
                    if summon_info.collectible_type_required.unwrap() == i && *collectibles_amount >= ammount_required {
                        *collectibles_amount -= ammount_required;

                        // Spawn summon
                        let position = Vec3::new(rng.gen_range(0.0..window.width()), rng.gen_range(0.0..window.height()), 0.0);
                        let summon = characters::Character {
                            last_shot: Some(Instant::now()),
                            summon: true,
                            last_damage: Instant::now(),
                            health: summon_info.max_health,
                            direction_vector: Vec3::ZERO,
                            type_index: j,
                            position_index: summon_positions.0.len(),
                            last_animation_frame: Instant::now() + Duration::new(0, rng.gen_range(animation::ANIMATION_START_OFFSET[0] * NANOS_PER_MILLIS..animation::ANIMATION_START_OFFSET[1] * NANOS_PER_MILLIS)),
                        };

                        summon_positions.0.push(Some(position));

                        let animation_information = summon_info.animation_information;
                        let texture: Handle<Image> = asset_server.load(animation_information.spritesheet_path);

                        let layout = TextureAtlasLayout::from_grid(
                            animation_information.sprite_size,
                            animation_information.frames,
                            1,
                            None,
                            None,
                        );
                        let layout = texture_atlas_layouts.add(layout);

                        commands.spawn(
                            (
                                summon,
                                Summon,
                                animation::AnimationSprite {
                                    frames: animation_information.frames,
                                    facing_right: animation_information.sprite_faces_right,
                                },
                                game::GameComponent,
                                SpriteSheetBundle {
                                    texture: texture,
                                    atlas: TextureAtlas {
                                        layout: layout,
                                        index: 0,
                                    },
                                    transform: Transform {
                                        translation: position,
                                        scale: Vec3::splat(art::SPRITE_SCALE),
                                        ..default()
                                    },
                                    visibility: Visibility::Visible,
                                    ..default()
                                },
                            )
                        );
                    }
                }
            }
        }
    }
}

fn move_summons(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut summon_query: Query<(&mut Transform, &mut characters::Character), With<Summon>>,
    summon_types: Res<characters::SummonTypes>,
    mut summon_positions: ResMut<SummonPositions>,
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let cursor_position = window.cursor_position();

    for (mut transform, mut summon) in summon_query.iter_mut() {
        let character_info = summon_types.0[summon.type_index];

        
        match character_info.summon_type.unwrap() {
            characters::SummonType::Sentry => continue, // Sentries cannot move
            characters::SummonType::Ranged => {
                if !buttons.pressed(MouseButton::Left) {
                    continue // Don't move ranged summons if their hotkey isn't being pressed
                }
            },
            characters::SummonType::Melee => {
                if !buttons.pressed(MouseButton::Right) {
                    continue // Don't move melee summons if their hotkey isn't being pressed
                }
            },
        }

        // Avoid other summons
        helpers::avoid_positions(&mut transform, &summon_positions.0, summon.position_index, SUMMON_RADIUS, &time);

        // Get the direction vector for the summon
        let direction_vector = match cursor_position {
            Some(cursor_position) => {
                let cursor_position = Vec3::new(cursor_position.x, (window.height() - cursor_position.y).abs(), 0.0);
                let difference = helpers::vector_subtract(&cursor_position, &transform.translation);

                if difference.length() > 32.0 {
                    difference.normalize()
                } else {
                    Vec3::ZERO
                }
            }
            None => summon.direction_vector, // If the mouse is out of bounds use the last direction vector
        };

        // Move summon
        transform.translation += direction_vector * character_info.speed * time.delta_seconds();

        summon.direction_vector = direction_vector;

         // Update enemy position
        if summon_positions.0[summon.position_index] != None {
            summon_positions.0[summon.position_index] = Some(transform.translation);
        }
    }
}

// Shoots ranged and sentry summons ranged attacks
fn shoot_ranged_attacks(
    mut commands: Commands,
    mut summon_query: Query<(&Transform, &mut characters::Character), With<Summon>>,
    enemy_positions: Res<enemies::EnemyPositions>,
    summon_types: Res<characters::SummonTypes>,
    projectile_types: Res<projectiles::ProjectileTypes>,
    asset_server: Res<AssetServer>,
) {
    for (transform, mut summon) in summon_query.iter_mut() {
        let character_info = summon_types.0[summon.type_index];

        // Melee enemies don't have projectiles
        if character_info.summon_type.unwrap() == characters::SummonType::Melee {
            continue;
        }

        // Check if the summon is ready to fire again
        if summon.last_shot.unwrap().elapsed().as_secs_f32() > summon_types.0[summon.type_index].firing_rate.unwrap() {
            summon.last_shot = Some(Instant::now());

            // Find the closest enemy to shoot at
            let mut minimum_distance: Option<f32> = None;
            let mut direction_vector: Option<Vec3> = None;
            for position in enemy_positions.0.iter() {
                if let Some(position) = position {
                    let difference = helpers::vector_subtract(&position, &transform.translation);
                    let distance = difference.length();
                    
                    // Update minimum distance and direction vector
                    match minimum_distance {
                        Some(min_distance) => {
                            if distance < min_distance {
                                minimum_distance = Some(distance);
                                direction_vector = Some(difference.normalize());
                            }
                        },
                        None => {
                            minimum_distance = Some(distance);
                            direction_vector = Some(difference.normalize());
                        },
                    }
                }
            }

            if let Some(direction_vector) = direction_vector {
                let projectile_type_index = summon_types.0[summon.type_index].projectile_types_index.unwrap();

                commands.spawn(
                    (
                        projectiles::Projectile {
                            direction_vector: direction_vector,
                            projectile_types_index: projectile_type_index,
                        },
                        game::GameComponent,
                        SpriteBundle {
                            transform: Transform {
                                translation: transform.translation,
                                scale: Vec3::splat(art::SPRITE_SCALE),
                                ..default()
                            },
                            texture: asset_server.load(projectile_types.0[projectile_type_index].sprite_information.sprite_path),
                            ..default()
                        }
                    )
                );
            }
        }

    }
}


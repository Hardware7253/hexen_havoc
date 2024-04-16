use rand::Rng;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::{Instant, Duration};

use crate::{game, art};
use game::{difficulty_settings, characters::EnemyTypes, characters, animation, NANOS_PER_MILLIS};
use super::{EnemyPositions, Enemy};

// Spawns all enemies for the wave
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    difficulty: Res<game::Difficulty>,
    enemy_types: Res<EnemyTypes>,
    asset_server: Res<AssetServer>,
    mut enemy_positions: ResMut<EnemyPositions>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    let spawn_radius = window.height() * difficulty_settings::SPAWN_RADIUS_BASE_MULTIPLIER;
    let spawn_diameter = (spawn_radius + difficulty.wave as f32 * difficulty_settings::SPAWN_RADIUS_SCALER) * 2.0;

    let padding = 20.0; // Minimum distance between an enemy spawn and the edge of the screen
   
    for _ in 0..difficulty.enemies as usize {

        // Spawn enemy at a random point around the edge of the screen
        // Brute force because I'm lazy
        let mut position: Option<Vec3> = None;
        while position == None {
            let test_position = Vec3::new(
                rng.gen_range((-spawn_diameter - padding)..(window.width() + spawn_diameter + padding)),
                rng.gen_range((-spawn_diameter - padding)..(window.height() + spawn_diameter + padding)),
                0.0,
            );

            if test_position.x > -padding && test_position.x < window.width() + padding {
                if test_position.y > -padding && test_position.y < window.height() + padding {
                    continue;
                }
            }

            position = Some(test_position);
        }
        let position = position.unwrap();

        let enemy_spawn_chance = difficulty.enemy_spawn_chance;

        // Select which enemy to spawn based on the enemy spawn chances defined in the diffuclty resource
        let mut enemy: Option<characters::Character> = None;
        for j in 0..enemy_spawn_chance.len() {
            if (100 - enemy_spawn_chance[j] as i8).abs() <= rng.gen_range(1..101) {
                enemy = Some(characters::Character {
                    last_shot: None,
                    summon: false,
                    last_damage: Instant::now(),
                    direction_vector: Vec3::ZERO,
                    health: enemy_types.0[j].max_health,
                    type_index: j,
                    position_index: enemy_positions.0.len(),
                    last_animation_frame: Instant::now() + Duration::new(0, rng.gen_range(animation::ANIMATION_START_OFFSET[0] * NANOS_PER_MILLIS..animation::ANIMATION_START_OFFSET[1] * NANOS_PER_MILLIS)),

                });
                break;
            }
        }

        

        if let Some(enemy) = enemy {
            enemy_positions.0.push(Some(position));

            let animation_information = enemy_types.0[enemy.type_index].animation_information;
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
                    enemy,
                    Enemy,
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
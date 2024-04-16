use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;
use crate::{AppState, art};
use super::{enemies, characters, keybinds, hitboxes, projectiles, helpers, GameState, WaveState, difficulty_settings};

#[derive(Component)]
pub struct Player {
    pub health: i32,
    speed: f32, // Current speed 
    direction_vector: Vec3, // Normaliszed direction vector
    movement_keys_pressed: bool, // True if any of the movement keys are pressed
    pub collectibles: [i32; characters::COLLECTIBLE_TYPES], // Different types of currencies the player has collected
}

const PLAYER_SPEED: f32 = 400.0; // Max speed
const PLAYER_MAX_HEALTH: i32 = 3;
const PLAYER_ACCELERATION: f32 = 8000.0; // Player acceleration and deacceleration

#[derive(Resource)]
struct HitInvulnaribilityTimer(Timer);

#[derive(Resource)]
struct PlayerAnimationTimer(Timer);

#[derive(Component)]
struct PlayerSprite {
    frames: usize,
    idle_animation: bool,
    facing_right: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HitInvulnaribilityTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .insert_resource(PlayerAnimationTimer(Timer::from_seconds(1.0 / art::ANIMATION_FPS, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::GameSetup), spawn_player)
            .add_systems(OnEnter(WaveState::Start), heal_player) // Heal player at the start of every wave
            .add_systems(Update, (move_player, collisions, check_for_game_over, select_animation, animate_player).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)));
    }
}

// Plays animations
fn animate_player(
    player_query: Query<&Transform, (With<Player>, Without<PlayerSprite>)>,
    mut animation_query: Query<(&mut Transform, &mut TextureAtlas, &PlayerSprite)>,
    mut animation_timer: ResMut<PlayerAnimationTimer>,
    time: Res<Time>,
) {
    animation_timer.0.tick(time.delta());
    if let Ok(player_transform) = player_query.get_single() {
        
        for (mut transform, mut texture_atlas, player_sprite) in animation_query.iter_mut() {

            transform.translation = player_transform.translation;
    
            if animation_timer.0.just_finished() {
                
                if texture_atlas.index != player_sprite.frames - 1 {
                    texture_atlas.index += 1;
                } else {
                    texture_atlas.index = 0;
                }
                
            }
        }
    };
    
}

// Selects the correct animation to play based on what the player is doing
fn select_animation(
    player_query: Query<&Player, Without<PlayerSprite>>,
    mut animation_query: Query<(&mut Visibility, &mut Transform, &PlayerSprite)>,
) {
    if let Ok(player) = player_query.get_single() {

        for (mut visibility, mut transform, player_sprite) in animation_query.iter_mut() {

            if !player.movement_keys_pressed { // When no movement keys are pressed play the idle animation
                if player_sprite.idle_animation {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            } else { // When movement keys are pressed play the move animation
                if player_sprite.idle_animation {
                    *visibility = Visibility::Hidden;
                } else {
                    *visibility = Visibility::Visible;
                }
            }

            // Rotate sprite so it faces in the direction it is moving
            let (rotation_1, rotation_2) = if player_sprite.facing_right {
                (PI, 0.0)
            } else {
                (0.0, PI)
            };

            if player.direction_vector.x > 0.0 {
                transform.rotation = Quat::from_rotation_y(rotation_1);
            } else if player.direction_vector.x < 0.0 {
                transform.rotation = Quat::from_rotation_y(rotation_2);
            }
        }
    }
}

fn heal_player(
    mut player_query: Query<&mut Player>
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        player.health = PLAYER_MAX_HEALTH;
    }
}

// Shows the game over menu if the player reaches 0 health
fn check_for_game_over(
    player_query: Query<&Player, Changed<Player>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(player) = player_query.get_single() {
        if player.health == 0 {
            next_game_state.set(GameState::GameOver);
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    let move_texture: Handle<Image> = asset_server.load(art::PLAYER_MOVE.spritesheet_path);
    let idle_texture: Handle<Image> = asset_server.load(art::PLAYER_IDLE.spritesheet_path);

    let move_layout = TextureAtlasLayout::from_grid(
        art::PLAYER_MOVE.sprite_size,
        art::PLAYER_MOVE.frames,
        1,
        None,
        None,
    );
    let move_layout = texture_atlas_layouts.add(move_layout);

    let idle_layout = TextureAtlasLayout::from_grid(
        art::PLAYER_IDLE.sprite_size,
        art::PLAYER_IDLE.frames,
        1,
        None,
        None,
    );
    let idle_layout = texture_atlas_layouts.add(idle_layout);
    
    commands.spawn(
        (
            super::GameComponent,
            Player {
                health: PLAYER_MAX_HEALTH,
                speed: 0.0,
                direction_vector: Vec3::splat(0.0),
                movement_keys_pressed: false,
                collectibles: difficulty_settings::STARTING_COLLECTIBLES,
            },
            Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        )
    );

    commands.spawn(
        (
            super::GameComponent,
            PlayerSprite {
                frames: art::PLAYER_MOVE.frames,
                idle_animation: false,
                facing_right: art::PLAYER_MOVE.sprite_faces_right,
            },
            SpriteSheetBundle {
                texture: move_texture,
                atlas: TextureAtlas {
                    layout: move_layout,
                    index: 0,
                },
                transform: Transform::from_scale(Vec3::splat(art::SPRITE_SCALE)),
                visibility: Visibility::Visible,
                ..default()
            },
        )
        
    );

    commands.spawn(
        (
            super::GameComponent,
            PlayerSprite {
                frames: art::PLAYER_IDLE.frames,
                idle_animation: true,
                facing_right: art::PLAYER_IDLE.sprite_faces_right,
            },
            SpriteSheetBundle {
                texture: idle_texture,
                atlas: TextureAtlas {
                    layout: idle_layout,
                    index: 0,
                },
                transform: Transform::from_scale(Vec3::splat(art::SPRITE_SCALE)),
                visibility: Visibility::Hidden,
                ..default()
            },
        )
        
    );
}

// Move player based off player inputs
fn move_player(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    let (mut player_transform, mut player) = player_query.get_single_mut().unwrap();

    // Movement vector based on the keys the player is pressing
    let mut direction_vector = Vec3::splat(0.0);

    let mut movement_keys_pressed = false;
    if keyboard_input.pressed(keybinds::PLAYER_UP) {
        movement_keys_pressed = true;
        direction_vector.y += 1.0;
    } if keyboard_input.pressed(keybinds::PLAYER_LEFT) {
        movement_keys_pressed = true;
        direction_vector.x -= 1.0;
    } if keyboard_input.pressed(keybinds::PLAYER_DOWN) {
        movement_keys_pressed = true;
        direction_vector.y -= 1.0;
    } if keyboard_input.pressed(keybinds::PLAYER_RIGHT) {
        movement_keys_pressed = true;
        direction_vector.x += 1.0;
    }

    player.movement_keys_pressed = movement_keys_pressed;
    
    // Normalise direction vector so the player doesn't move faster diagonally
    direction_vector = direction_vector.normalize_or_zero();
    
    // Acceleration and deacceleration
    if direction_vector != Vec3::splat(0.0) {
        player.direction_vector = direction_vector;

        player.speed += PLAYER_ACCELERATION * time.delta_seconds();
        if player.speed > PLAYER_SPEED {
            player.speed = PLAYER_SPEED;
        }
    } else {
        player.speed -= PLAYER_ACCELERATION * time.delta_seconds();
        if player.speed < 0.0 {
            player.speed = 0.0
        }
    }

    let speed_multiplier = player.speed * time.delta_seconds();
    let new_position = player_transform.translation + player.direction_vector * speed_multiplier;
    
    if helpers::is_on_screen(&new_position, &window) {
        player_transform.translation = new_position;
    } else {
        player.direction_vector = helpers::vector_multiply(&direction_vector, &Vec3::new(-1.0, -1.0, 0.0));
        player_transform.translation += player.direction_vector * speed_multiplier;
    }

}

// Handle collisions between player and enemies, and enemy projectiles
fn collisions(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    enemy_query: Query<(&Transform, &characters::Character), With<enemies::Enemy>>,
    projectile_query: Query<(&Transform, &projectiles::Projectile, Entity)>,
    projectile_types: Res<projectiles::ProjectileTypes>,
    enemy_types: Res<characters::EnemyTypes>,
    mut invulnarability_timer: ResMut<HitInvulnaribilityTimer>,
    time: Res<Time>,
) {
    let (player_transfrom, mut player) = player_query.get_single_mut().unwrap();
    invulnarability_timer.0.tick(time.delta());

    let mut collision = false;

    // Test for contact collisions with enemies
    for (enemy_transform, enemy) in enemy_query.iter() {
        let enemy_hitbox = enemy_types.0[enemy.type_index].animation_information.hitbox;

        if hitboxes::are_hitboxes_colliding(&art::PLAYER_HITBOX, &player_transfrom.translation, &enemy_hitbox, &enemy_transform.translation) {
            collision = true;
        }
    }

    // Test for projectile collisions
    for (projectile_transform, projectile, entity) in projectile_query.iter() {
        let projectile_type = &projectile_types.0[projectile.projectile_types_index];

        if projectile_type.enemy { // Only damage the player if it is an enemy projectile
            if hitboxes::are_hitboxes_colliding(&art::PLAYER_HITBOX, &player_transfrom.translation, &projectile_type.sprite_information.hitbox, &projectile_transform.translation) {
                commands.entity(entity).despawn();
                collision = true;
            }
        }
    }

    // Only damage the player if they don't have invulnarability
    // The player doesn't care about the contact damage or the projectile damage, they always loose one health
    if collision & invulnarability_timer.0.finished() {
        player.health -= 1;
        invulnarability_timer.0.reset();
    }
}
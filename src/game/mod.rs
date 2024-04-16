use bevy::prelude::*;
use crate::AppState;

pub mod player;
pub mod keybinds;
pub mod enemies;
pub mod hitboxes;
pub mod helpers;
pub mod projectiles;
pub mod characters;
pub mod summons;
pub mod damage;
pub mod collectibles;
pub mod animation;

pub const NANOS_PER_MILLIS: u32 = 1000000;

pub const WAVE_COUNTDOWN_SECONDS: u8 = 3; // Seconds inbetween waves

pub mod difficulty_settings {
    use super::characters;
    
    pub const STARTING_COLLECTIBLES: [i32; characters::COLLECTIBLE_TYPES] = [12, 0, 0, 0]; // Starting collectibles for the player

    pub const STARTING_ENEMIES: f32 = 9.0; // Number of enemies on wave 0
    pub const ENEMIES_MULTIPLIER: f32 = 1.2; // Multiply the number of enemies from last wave by this number to get the new number of enemies for a wave
    pub const MIN_TRICKLE_AMMOUNT: u8 = 18; // Minimum percentage that can trickle from an easier enmies spawn rate into a harder enemies spawn rate

    // Defines how large the enemy spawn radius is
    pub const SPAWN_RADIUS_BASE_MULTIPLIER: f32 = 0.25; // The spawn radius is atleast as big as the screen height multiplied by this number
    pub const SPAWN_RADIUS_SCALER: f32 = 20.0; // This number multiplied by the wave number is added to the base spawn radius
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(Component)]
pub struct GameComponent;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum WaveState {
    #[default]
    Start,
    Fight,
}

#[derive(Event)]
pub struct ScoreIncrease;

#[derive(Resource, Debug)]
pub struct Difficulty { // Containts difficulty information for the current wave
    pub wave: u32,
    pub enemy_spawn_chance: [u8; characters::ENEMY_TYPES], // Enemy spawn chance by enemy type, order if this array is the same as the one from characters::EnemyTypes
    pub enemies: f32, // Number of enemies to spawn in the wave, enemies have spawn chances so slightly less enemies may be spawned
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ScoreIncrease>()
        .init_state::<GameState>()
        .init_state::<WaveState>()

        .add_plugins((player::PlayerPlugin, enemies::EnemyPlugin, projectiles::ProjectilePlugin, characters::CharacterPlugin, summons::SummonPlugin, damage::DamagePlugin, collectibles::CollectiblePlugin, animation::AnimationPlugin))

        .add_systems(Update, (next_wave, dead_summons_end_game).run_if(in_state(AppState::Game)).run_if(in_state(WaveState::Fight)).run_if(in_state(GameState::Running)))
        .add_systems(OnExit(WaveState::Fight), update_difficulty) // Update difficulty inbetween waves

        .add_systems(OnEnter(AppState::GameSetup), init_game)
        .add_systems(OnEnter(AppState::GameCleanup), despawn_game_components);
    }
}

// Goes to the next wave once all the enemies are dead
fn next_wave(
    enemy_query: Query<&enemies::Enemy>,
    mut next_wave_state: ResMut<NextState<WaveState>>
) {
    if enemy_query.is_empty() {
        next_wave_state.set(WaveState::Start);
    }
}

// Ends the game if all your summons die
fn dead_summons_end_game(
    character_query: Query<&summons::Summon>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if character_query.is_empty() {
        next_game_state.set(GameState::GameOver)
    }
}

// This function increases the game difficulty everytime it is called
fn update_difficulty(mut difficulty: ResMut<Difficulty>) {   

    difficulty.wave += 1;
    let mut trickle_ammount = difficulty_settings::MIN_TRICKLE_AMMOUNT;
    let enemy_spawn_chance = &mut difficulty.enemy_spawn_chance;

    let max_value = max_value(enemy_spawn_chance.to_vec()).unwrap();

    // Trickle spawn rates from easier enemies into harder enemies
    for i in (0..enemy_spawn_chance.len()).rev() {
        if enemy_spawn_chance[i] > max_value / 2 && i != enemy_spawn_chance.len() - 1 {
            if enemy_spawn_chance[i] >= trickle_ammount {
                enemy_spawn_chance[i] -= trickle_ammount;
                enemy_spawn_chance[i + 1] += trickle_ammount;
                trickle_ammount *= 2;
            }
        }
    }

    // Increase the number of enemies spawned
    difficulty.enemies *= difficulty_settings::ENEMIES_MULTIPLIER;
}

// Returns the max value in a u8 vec
fn max_value(values: Vec<u8>) -> Result<u8, ()> {
    
    if values.is_empty() {
        return Err(());
    }
    
    let mut max_value = values[0];
    for value in values.iter() {
        if *value > max_value {
            max_value = *value
        }
    }
    Ok(max_value)
}


// Game resources are re-inserted every time the game is setup
// So the score, currency, and wave always start at 0
fn init_game(mut commands: Commands, mut next_wave_state: ResMut<NextState<WaveState>>) {

    let mut enemy_spawn_chance_vec = vec![100];
    for _ in 0..characters::ENEMY_TYPES - 1 {
        enemy_spawn_chance_vec.push(0);
    }

    commands.insert_resource(
        Difficulty {
            wave: 0,
            enemy_spawn_chance: enemy_spawn_chance_vec.try_into().unwrap(),
            enemies: difficulty_settings::STARTING_ENEMIES,
        }
    );
    next_wave_state.set(WaveState::Start);
}

// Despawn all game components
fn despawn_game_components(mut commands: Commands, game_components_query: Query<Entity, With<GameComponent>>) {
    for entity in game_components_query.iter() {
        commands.entity(entity).despawn();
    }
}
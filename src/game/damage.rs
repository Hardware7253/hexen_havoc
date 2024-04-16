use bevy::prelude::*;
use std::time::Instant;
use crate::{AppState, art};
use super::{enemies, summons, characters, projectiles, hitboxes, WaveState, GameState, collectibles, GameComponent};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (deal_damage, despawn_characters).run_if(in_state(AppState::Game)).run_if(in_state(WaveState::Fight)).run_if(in_state(GameState::Running)).before(enemies::move_enemies));
            
    }
}


// Despawn characters after they have taken too much damage
// Spawn currency at enemy death location
fn despawn_characters(
    mut commands: Commands,
    enemy_query: Query<(&characters::Character, Entity), (With<enemies::Enemy>, Without<summons::Summon>)>,
    summon_query: Query<(&characters::Character, Entity), With<summons::Summon>>,
    mut summon_positions: ResMut<summons::SummonPositions>,
    mut enemy_positions: ResMut<enemies::EnemyPositions>,
    enemy_types: Res<characters::EnemyTypes>,
    asset_server: Res<AssetServer>,
) {
    for (summon, entity) in summon_query.iter() {
        if summon.health <= 0 {
            summon_positions.0[summon.position_index] = None;
            commands.entity(entity).despawn();
        }
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0 {
            let mut currency_spawn_position = enemy_positions.0[enemy.position_index].unwrap();
            currency_spawn_position.z = -1.0;
            let enemy_info = enemy_types.0[enemy.type_index];

            enemy_positions.0[enemy.position_index] = None;
            commands.entity(entity).despawn();

            // When an enemy is despawned drop the appropriate ammount of currency
            commands.spawn(
                (
                    collectibles::Collectible {
                        collectible_type: enemy_info.drop_collectible.unwrap(),
                        hitbox: enemy_info.drop_collectible_sprite.unwrap().hitbox,
                    },
                    GameComponent,
                    SpriteBundle {
                        transform: Transform {
                            translation: currency_spawn_position,
                            scale: Vec3::splat(art::SPRITE_SCALE),
                            ..default()
                        },
                        texture: asset_server.load(enemy_info.drop_collectible_sprite.unwrap().sprite_path),
                        ..default()
                    }
                )
            );
        }
    }
}

// Deals projectile and contact damage to summons and enemies, respects invulnarability
fn deal_damage(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut characters::Character), (With<enemies::Enemy>, Without<summons::Summon>)>,
    mut summon_query: Query<(&Transform, &mut characters::Character), With<summons::Summon>>,
    projectile_query: Query<(&Transform, &projectiles::Projectile, Entity)>,
    projectile_types: Res<projectiles::ProjectileTypes>,
    summon_types: Res<characters::SummonTypes>,
    enemy_types: Res<characters::EnemyTypes>,
) {
    
    for (projectile_transform, projectile, projectile_entity) in projectile_query.iter() {
        let projectile_info = &projectile_types.0[projectile.projectile_types_index];

        if projectile_info.enemy { // Deal damage from enemy projectiles to summons
            for (summon_transform, mut summon) in summon_query.iter_mut() {
                let summon_info = &summon_types.0[summon.type_index];

                projectile_hit_character(
                    &mut commands,
                    projectile_entity, &projectile_info, &projectile_transform.translation,
                    &summon_info, &mut summon, &summon_transform.translation
                );
            }
        } else { // Deal damage from summon projectiles to enemies
            for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
                let enemy_info = &enemy_types.0[enemy.type_index];

                projectile_hit_character(
                    &mut commands,
                    projectile_entity, &projectile_info, &projectile_transform.translation,
                    &enemy_info, &mut enemy, &enemy_transform.translation
                );
            }
        }
    }

    // Deal contact damage between summons and enemies
    for (summon_transform, mut summon) in summon_query.iter_mut() {
        let summon_info = &summon_types.0[summon.type_index];
        for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let enemy_info = &enemy_types.0[enemy.type_index];

            // Check if summon and enemy are colliding
            if hitboxes::are_hitboxes_colliding(&summon_info.animation_information.hitbox, &summon_transform.translation, &enemy_info.animation_information.hitbox, &enemy_transform.translation) {

                // Deal damage to summons
                if summon.last_damage.elapsed().as_secs_f32() > summon_info.invulnarability_time {
                    summon.last_damage = Instant::now();
                    summon.health -= enemy_info.contact_damange;
                }

                // Deal damage to enemies
                if enemy.last_damage.elapsed().as_secs_f32() > enemy_info.invulnarability_time {
                    enemy.last_damage = Instant::now();
                    enemy.health -= summon_info.contact_damange;
                }
            }
        }
    }
}

// Detect if a projectile hits a character
// Damage character when this happens, respect invulnarability
// Despawn projectile
fn projectile_hit_character(
    commands: &mut Commands,
    projectile_entity: Entity,
    projectile_info: &projectiles::ProjectileInfo,
    projectile_position: &Vec3,
    character_info: &characters::CharacterInfo,
    character: &mut characters::Character,
    character_position: &Vec3,
) {
    if hitboxes::are_hitboxes_colliding(&projectile_info.sprite_information.hitbox, &projectile_position, &character_info.animation_information.hitbox, &character_position) {

        // Despawn projectile if it isn't a piercing projectile
        if !projectile_info.piercing {
            commands.entity(projectile_entity).despawn();
        }
        
        // Only damage character if it is outside of it's invulnarability time
        if character.last_damage.elapsed().as_secs_f32() > character_info.invulnarability_time {
            character.last_damage = Instant::now();
            character.health -= projectile_info.damage;
        }
    }
}
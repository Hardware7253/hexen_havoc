use bevy::prelude::*;
use std::time::Instant;
use crate::art;

#[derive(Component, Debug)]
pub struct Character {
    pub health: i32, // Current health
    pub last_shot: Option<Instant>, // Last instant the character shot
    pub last_damage: Instant, // Last instant the character was damaged
    pub direction_vector: Vec3, // Most recent direction vector of the character
    pub type_index: usize, // Index in it's respective enemy or summoner type resource
    pub summon: bool,
    pub position_index: usize, // Index in it's respective enemy or summoner position resource
    pub last_animation_frame: Instant, // Last time the animation frame switched
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SummonType {
    Sentry,
    Melee,
    Ranged,
}

#[derive(Copy, Clone)]
pub struct CharacterInfo {
    pub max_health: i32,
    pub speed: f32,

    pub summon_type: Option<SummonType>, // None of the character is an enemy

    // Fields related to projectiles have options
    // If no projectile information is provided melee is assumed
    pub firing_rate: Option<f32>, // Time to wait in seconds between shooting
    pub projectile_types_index: Option<usize>, // Index for the bullet in the projectile type resource

    pub contact_damange: i32,
    pub invulnarability_time: f32, // Invulnaribility time inbetween hits (in seconds)

    pub collectible_type_required: Option<usize>, // Type of collectible the summon needs to spawn
    pub collectible_ammount_required: Option<i32>, // Ammount of collectible the summon needs to spawn

    pub drop_collectible: Option<usize>, // Type of collectible the enemy drops
    pub drop_collectible_sprite: Option<art::SpriteInformation>,

    pub animation_information: art::AnimationSpriteInformation,
}

pub const ENEMY_TYPES: usize = 4;
pub const SUMMON_TYPES: usize = 4;
pub const COLLECTIBLE_TYPES: usize = 4;

// Enemies in vec are ordered in terms of difficulty
#[derive(Resource)]
pub struct EnemyTypes(pub [CharacterInfo; ENEMY_TYPES]);

#[derive(Resource)]
pub struct SummonTypes(pub [CharacterInfo; SUMMON_TYPES]);

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EnemyTypes(

                    // Enemies ordered in terms of accending difficulty
                    [
                        CharacterInfo {
                            max_health: 5,
                            speed: 100.0,
                            summon_type: None,
                            firing_rate: Some(2.5),
                            projectile_types_index: Some(0),
                            contact_damange: 1,
                            invulnarability_time: 0.0,
                            collectible_type_required: None,
                            collectible_ammount_required: None,
                            drop_collectible: Some(0),
                            drop_collectible_sprite: Some(art::BONE_COLLECTIBLE_SPRITE),
                            animation_information: art::ENEMY_ANIMATION,
                        },


                        CharacterInfo {
                            max_health: 7,
                            speed: 120.0,
                            summon_type: None,
                            firing_rate: Some(2.3),
                            projectile_types_index: Some(1),
                            contact_damange: 2,
                            invulnarability_time: 0.0,
                            collectible_type_required: None,
                            collectible_ammount_required: None,
                            drop_collectible: Some(1),
                            drop_collectible_sprite: Some(art::BLOOD_COLLECTIBLE_SPRITE),
                            animation_information: art::ENEMY_ANIMATION,
                        },

                        CharacterInfo {
                            max_health: 9,
                            speed: 140.0,
                            summon_type: None,
                            firing_rate: Some(2.0),
                            projectile_types_index: Some(2),
                            contact_damange: 1,
                            invulnarability_time: 0.0,
                            collectible_type_required: None,
                            collectible_ammount_required: None,
                            drop_collectible: Some(2),
                            drop_collectible_sprite: Some(art::CRYSTAL_COLLECTIBLE_1_SPRITE),
                            animation_information: art::ENEMY_ANIMATION,
                        },

                        CharacterInfo {
                            max_health: 11,
                            speed: 170.0,
                            summon_type: None,
                            firing_rate: Some(1.8),
                            projectile_types_index: Some(3),
                            contact_damange: 1,
                            invulnarability_time: 0.0,
                            collectible_type_required: None,
                            collectible_ammount_required: None,
                            drop_collectible: Some(3),
                            drop_collectible_sprite: Some(art::CRYSTAL_COLLECTIBLE_2_SPRITE),
                            animation_information: art::ENEMY_ANIMATION,
                        },
                    ]
                )
            )

            .insert_resource(SummonTypes(
                [   
                    CharacterInfo {
                        max_health: 2,
                        speed: 800.0,
                        summon_type: Some(SummonType::Ranged),
                        firing_rate: Some(1.0),
                        projectile_types_index: Some(4),
                        contact_damange: 1,
                        invulnarability_time: 1.0,
                        collectible_type_required: Some(0),
                        collectible_ammount_required: Some(5),
                        drop_collectible: None,
                        drop_collectible_sprite: None,
                        animation_information: art::EYE_SUMMON_ANIMATION,
                    },

                    CharacterInfo {
                        max_health: 40,
                        speed: 800.0,
                        summon_type: Some(SummonType::Melee),
                        firing_rate: None,
                        projectile_types_index: None,
                        contact_damange: 5,
                        invulnarability_time: 0.0,
                        collectible_type_required: Some(1),
                        collectible_ammount_required: Some(4),
                        drop_collectible: None,
                        drop_collectible_sprite: None,
                        animation_information: art::GHOST_SUMMON_ANIMATION,
                    },

                    CharacterInfo {
                        max_health: 4,
                        speed: 800.0,
                        summon_type: Some(SummonType::Ranged),
                        firing_rate: Some(0.8),
                        projectile_types_index: Some(5),
                        contact_damange: 1,
                        invulnarability_time: 1.0,
                        collectible_type_required: Some(2),
                        collectible_ammount_required: Some(8),
                        drop_collectible: None,
                        drop_collectible_sprite: None,
                        animation_information: art::FLAMEBALL_SUMMON_ANIMATION,
                    },

                    CharacterInfo {
                        max_health: 80,
                        speed: 800.0,
                        summon_type: Some(SummonType::Melee),
                        firing_rate: None,
                        projectile_types_index: None,
                        contact_damange: 10,
                        invulnarability_time: 0.0,
                        collectible_type_required: Some(3),
                        collectible_ammount_required: Some(10),
                        drop_collectible: None,
                        drop_collectible_sprite: None,
                        animation_information: art::DWARVE_SUMMON_ANIMATION,
                    },
                ]
            )
        );
    }
}
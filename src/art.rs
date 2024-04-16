use bevy::prelude::*;

// All spritesheets are horizontal strips
// Hitboxes are included here becuase they are closely related to the art

#[derive(Copy, Clone)]
pub struct AnimationSpriteInformation {
    pub spritesheet_path: &'static str,
    pub sprite_faces_right: bool,
    pub sprite_size: Vec2,
    pub hitbox: Vec2,
    pub frames: usize,
}

#[derive(Copy, Clone)]
pub struct SpriteInformation {
    pub sprite_path: &'static str,
    pub hitbox: Vec2,
}

pub const SPRITE_SCALE: f32 = 1.0;

pub const BACKGROUND_PATH: &'static str = "sprites/background.png";
pub const BACKGROUND_SIZE: Vec2 = Vec2::new(420.0, 297.0); // Pixel size of the background image

// Projectile sprites
pub const FRIENDLY_PROJECTILE_1_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/projectiles/projectile_yellow.png",
    hitbox: Vec2::new(17.0 * SPRITE_SCALE, 17.0 * SPRITE_SCALE),
};

pub const FRIENDLY_PROJECTILE_2_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/projectiles/projectile_orange.png",
    hitbox: Vec2::new(15.0 * SPRITE_SCALE, 15.0 * SPRITE_SCALE),
};

pub const ENEMY_PROJECTILE_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/projectiles/projectile_red.png",
    hitbox: Vec2::new(16.0 * SPRITE_SCALE, 16.0 * SPRITE_SCALE),
};


pub const HEALTH_SPRITE_PATH: &'static str = "sprites/heart.png";

pub const ANIMATION_FPS: f32 = 12.0;

// Player animations
pub const PLAYER_HITBOX: Vec2 = Vec2::new(58.0 * SPRITE_SCALE, 100.0 * SPRITE_SCALE);

pub const PLAYER_IDLE: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/player_idle_spritesheet.png",
    sprite_faces_right: true,
    sprite_size: Vec2::new(208.0, 208.0),
    hitbox: PLAYER_HITBOX,
    frames: 7,
};

pub const PLAYER_MOVE: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/player_move_spritesheet.png",
    sprite_faces_right: true,
    sprite_size: Vec2::new(201.0, 218.0),
    hitbox: PLAYER_HITBOX,
    frames: 7,
};

// Enemy animation
pub const ENEMY_ANIMATION: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/enemy_spritesheet.png",
    sprite_faces_right: false,
    sprite_size: Vec2::new(170.0, 220.0),
    hitbox: Vec2::new(38.0 * SPRITE_SCALE, 157.0 * SPRITE_SCALE),
    frames: 6,
};

// Summon animations
pub const EYE_SUMMON_ANIMATION: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/summons/eye_spritesheet.png",
    sprite_faces_right: true,
    sprite_size: Vec2::new(99.0, 57.0),
    hitbox: Vec2::new(22.0 * SPRITE_SCALE, 22.0 * SPRITE_SCALE),
    frames: 6,
};

pub const DWARVE_SUMMON_ANIMATION: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/summons/dwarve_spritesheet.png",
    sprite_faces_right: true,
    sprite_size: Vec2::new(199.0, 214.0),
    hitbox: Vec2::new(62.0 * SPRITE_SCALE, 120.0 * SPRITE_SCALE),
    frames: 6,
};

pub const GHOST_SUMMON_ANIMATION: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/summons/ghost_spritesheet.png",
    sprite_faces_right: true,
    sprite_size: Vec2::new(112.0, 145.0),
    hitbox: Vec2::new(48.0 * SPRITE_SCALE, 76.0 * SPRITE_SCALE),
    frames: 5,
};

pub const FLAMEBALL_SUMMON_ANIMATION: AnimationSpriteInformation = AnimationSpriteInformation {
    spritesheet_path: "sprites/summons/flameball_spritesheet.png",
    sprite_faces_right: true,
    sprite_size: Vec2::new(244.0, 213.0),
    hitbox: Vec2::new(86.0 * SPRITE_SCALE, 98.0 * SPRITE_SCALE),
    frames: 6,
};

// Collectible sprites
pub const BONE_COLLECTIBLE_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/collectibles/bone_collectible.png",
    hitbox: Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE),
};

pub const CRYSTAL_COLLECTIBLE_1_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/collectibles/crystal_collectible_1.png",
    hitbox: Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE),
};

pub const CRYSTAL_COLLECTIBLE_2_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/collectibles/crystal_collectible_2.png",
    hitbox: Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE),
};

pub const BLOOD_COLLECTIBLE_SPRITE: SpriteInformation = SpriteInformation {
    sprite_path: "sprites/collectibles/blood_collectible.png",
    hitbox: Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE),
};
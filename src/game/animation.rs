use bevy::prelude::*;
use std::time::Instant;
use std::f32::consts::PI;
use crate::{AppState, art};
use crate::game::{characters, GameState};

pub const ANIMATION_START_OFFSET: [u32; 2] = [0, 300]; // Used so all the animations don't play at the same time, array defines a random offset in milliseconds

#[derive(Component)]
pub struct AnimationSprite {
    pub frames: usize,
    pub facing_right: bool,
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, animate.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)));
    }
}

// Animates all enemies and summons
fn animate(
    mut animation_query: Query<(&AnimationSprite, &mut Transform, &mut TextureAtlas, &mut characters::Character)>,
) {
    for (animation_sprite, mut transform, mut texture_atlas, mut character) in animation_query.iter_mut() {


        if character.last_animation_frame.elapsed().as_secs_f32() > 1.0 / art::ANIMATION_FPS {
            character.last_animation_frame = Instant::now();
            if texture_atlas.index != animation_sprite.frames - 1 {
                texture_atlas.index += 1;
            } else {
                texture_atlas.index = 0;
            }
        }

        // Rotate sprite so it faces in the direction it is moving
        let (rotation_1, rotation_2) = if animation_sprite.facing_right {
            (0.0, PI)
        } else {
            (PI, 0.0)
        };

        if character.direction_vector.x > 0.0 {
            transform.rotation = Quat::from_rotation_y(rotation_1);
        } else if character.direction_vector.x < 0.0 {
            transform.rotation = Quat::from_rotation_y(rotation_2);
        }
    }
}
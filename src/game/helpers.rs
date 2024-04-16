use bevy::prelude::*;
use crate::game::helpers;

// Returns a - b
pub fn vector_subtract(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)
}

// Returns a * b
pub fn vector_multiply(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

// Return true if a point is on screen
pub fn is_on_screen(position: &Vec3, window: &Window) -> bool {
    position.x > 0.0 && position.x < window.width() &&
    position.y > 0.0 && position.y < window.height()
}

// Automatically moves a transform to avoid positions defined in the vec
// Only avoids when the transform and position are within 2 * (avoid_radius) of each other
// The transform given should be one of the components whose position is contained in (positions)
// Returns true if the given transform was altered to avoid the positions
pub fn avoid_positions(
    transform: &mut Transform,
    positions: &Vec<Option<Vec3>>,
    positions_index: usize,
    avoid_radius: f32,
    time: &Res<Time>,
) -> bool {
    // Find avoid distance and direction so the enemy avoids other enemies
    let mut avoid_direction_vector = Vec3::ZERO;
    let mut avoid_distance: f32 = 0.0;
    let mut translation_changed = false;
    for (i, position) in positions.iter().enumerate() {
        if let Some(position) = position {
            if i == positions_index {
                continue;
            }
    
            let center_difference = helpers::vector_subtract(&transform.translation, &position);
            let clip_distance = center_difference.length() - avoid_radius * 2.0; // Positive for no clip, negative for clip
            if clip_distance < 0.0 {
                avoid_direction_vector += center_difference;
                avoid_distance = clip_distance.abs();
            }
    
            transform.translation += avoid_direction_vector * avoid_distance * time.delta_seconds();
    
            if !translation_changed {
                translation_changed = avoid_direction_vector != Vec3::ZERO  && avoid_distance != 0.0;
            }
        }
    }
    translation_changed
}
use bevy::prelude::*;
use crate::{AppState, art};
use super::{GameState, player, hitboxes};

#[derive(Component)]
pub struct Collectible {
    pub collectible_type: usize,
    pub hitbox: Vec2,
}
pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, detect_collisions.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)));
    }
}

// Detects collisions between collectible and player so that the players currency can be increased
fn detect_collisions(
    mut commands: Commands,
    collectible_query: Query<(&Transform, &Collectible, Entity)>,
    mut player_query: Query<(&Transform, &mut player::Player)>
) {

    let (player_transform, mut player) = player_query.get_single_mut().unwrap();

    for (collectible_transform, collectible, entity) in collectible_query.iter() {
        if hitboxes::are_hitboxes_colliding(&art::PLAYER_HITBOX, &player_transform.translation, &collectible.hitbox, &collectible_transform.translation) {
            player.collectibles[collectible.collectible_type] += 1;
            commands.entity(entity).despawn();
        }
    }
}




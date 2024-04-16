
use bevy::prelude::*;

// A rectangle defined by it's min and max x and y positions
#[derive(Debug)]
struct Rectangle {
    x_max: f32,
    x_min: f32,
    y_max: f32,
    y_min: f32,
}

// Creats a rectangle from a hitbox
fn hitbox_to_rectangle(hitbox: &Vec2, position: &Vec3) -> Rectangle {
    let half_hitbox_width = hitbox.x / 2.0;
    let half_hitbox_height = hitbox.y / 2.0;

    Rectangle {
        x_max: position.x + half_hitbox_width,
        x_min: position.x - half_hitbox_width,
        y_max: position.y + half_hitbox_height,
        y_min: position.y - half_hitbox_height,
    }
}

// Returns true if hitboxes are colliding
pub fn are_hitboxes_colliding(a: &Vec2, a_pos: &Vec3, b: &Vec2, b_pos: &Vec3) -> bool {

    // Test for rectangle collision before checking circles
    // Because rectangle is a faster approximation
    let rect_a = hitbox_to_rectangle(&a, &a_pos);
    let rect_b = hitbox_to_rectangle(&b, &b_pos);

    let x_collision = a_pos.x < b_pos.x && rect_a.x_max > rect_b.x_min ||
        a_pos.x > b_pos.x && rect_a.x_min < rect_b.x_max ||
        a_pos.x == b_pos.x;

    let y_collision = a_pos.y < b_pos.y && rect_a.y_max > rect_b.y_min ||
        a_pos.y > b_pos.y && rect_a.y_min < rect_b.y_max ||
        a_pos.y == b_pos.y;

    x_collision && y_collision
}
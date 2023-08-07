use bevy::prelude::*;
use crate::player::*;
use crate::structures::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct CollisionComponent {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl CollisionComponent {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        CollisionComponent { x, y, w, h }
    }

    pub fn new_from_component(component: &dyn Collisionable) -> Self {
        let (x, y, w, h) = component.get_hitbox();
        CollisionComponent::new(x, y, w, h)
    }

    pub fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.w, self.h)
    }
}



pub trait Collisionable {
    fn get_pos(&self) -> (i32, i32);
    fn get_hitbox(&self) -> (i32, i32, i32, i32);

    fn would_collide(&self, x: i32, y:i32, other: &CollisionComponent) -> bool {
        let (x1, y1, w1, h1) = (x, y, self.get_hitbox().2, self.get_hitbox().3);
        let (x2, y2, w2, h2) = other.get_hitbox();

        let (x1, y1) = get_position_from_center_to_corner(x1, y1, w1, h1);
        let (x2, y2) = get_position_from_center_to_corner(x2, y2, w2, h2);

        let x1_min = x1;
        let x1_max = x1 + w1;
        let y1_min = y1;
        let y1_max = y1 + h1;

        let x2_min = x2;
        let x2_max = x2 + w2;
        let y2_min = y2;
        let y2_max = y2 + h2;

        if x1_max < x2_min || x1_min > x2_max {
            return false;
        }

        if y1_max < y2_min || y1_min > y2_max {
            return false;
        }

        true
        
    }

    fn get_collision_component(&self) -> CollisionComponent {
        let (x, y, w, h) = self.get_hitbox();
        CollisionComponent::new(x, y, w, h)
    }
}

fn get_position_from_center_to_corner(x: i32, y: i32, w: i32, h: i32) -> (i32, i32) {
    (x - w/2, y - h/2)
}
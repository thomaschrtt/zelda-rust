use bevy::prelude::*;

#[derive(Component, Clone)]
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

    pub fn update_hitbox(&mut self, component: &dyn Collisionable) {
        let (x, y, w, h) = component.get_hitbox();
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
}

impl Collisionable for CollisionComponent {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.w, self.h)
    }
}


pub trait Collisionable {
    fn get_pos(&self) -> (i32, i32);
    fn get_hitbox(&self) -> (i32, i32, i32, i32);

    fn would_collide(&self, x: i32, y:i32, other: &CollisionComponent) -> bool {
        let (x1, y1, w1, h1) = (x, y, self.get_hitbox().2, self.get_hitbox().3);
        let (x2, y2, w2, h2) = other.get_hitbox();

        are_overlapping(x1, y1, w1, h1, x2, y2, w2, h2)
        
    }

    fn would_collide_with(&self, other: &dyn Collisionable) -> bool {
        let self_pos: (i32, i32) = self.get_pos();
        self.would_collide(self_pos.0, self_pos.1, &other.get_collision_component())
    }

    fn get_collision_component(&self) -> CollisionComponent {
        let (x, y, w, h) = self.get_hitbox();
        CollisionComponent::new(x, y, w, h)
    }
    
}

pub fn get_position_from_center_to_corner(x: i32, y: i32, w: i32, h: i32) -> (i32, i32) {
    (x - w/2, y - h/2)
}

pub fn are_overlapping(x1: i32, y1: i32, w1: i32, h1: i32, 
                       x2: i32, y2: i32, w2: i32, h2: i32) -> bool {
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

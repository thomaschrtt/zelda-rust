use bevy::prelude::*;
use crate::{ennemies::*, player::PlayerFacingDirection};

#[derive(Component, Clone)]
pub struct CollisionComponent {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(PartialEq)]
pub enum RelativePosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl CollisionComponent {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        CollisionComponent { x, y, w, h }
    }

    pub fn new_from_component(component: &dyn Collisionable) -> Self {
        let (x, y, w, h) = component.get_hitbox();
        CollisionComponent::new(x, y, w, h)
    }

    pub fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    pub fn update_hitbox(&mut self, component: &dyn Collisionable) {
        let (x, y, w, h) = component.get_hitbox();
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl Collisionable for CollisionComponent {
    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
}


pub trait Collisionable {
    fn get_pos(&self) -> (f32, f32);
    fn get_hitbox(&self) -> (f32, f32, f32, f32);

    fn would_collide(&self, x: f32, y:f32, other: &CollisionComponent) -> bool {
        let (x1, y1, w1, h1) = (x, y, self.get_hitbox().2, self.get_hitbox().3);
        let (x2, y2, w2, h2) = other.get_hitbox();

        are_overlapping(x1, y1, w1, h1, x2, y2, w2, h2)
        
    }

    fn would_collide_with(&self, other: &dyn Collisionable) -> bool {
        let self_pos: (f32, f32) = self.get_pos();
        self.would_collide(self_pos.0, self_pos.1, &other.get_collision_component())
    }

    fn get_collision_component(&self) -> CollisionComponent {
        let (x, y, w, h) = self.get_hitbox();
        CollisionComponent::new(x, y, w, h)
    }

    fn get_relative_position(&self, other: &dyn Collisionable) -> Option<RelativePosition> {
        let (x1, y1, w1, h1) = self.get_hitbox();
        let (x2, y2, w2, h2) = other.get_hitbox();

        get_relative_position(x1, y1, w1, h1, x2, y2, w2, h2)
    }
    
}

pub fn get_position_from_center_to_corner(x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
    (x - w/2., y - h/2.)
}

pub fn are_overlapping(x1: f32, y1: f32, w1: f32, h1: f32, 
                       x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
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

pub fn get_relative_position(x1: f32, y1: f32, w1: f32, h1: f32, 
                             x2: f32, y2: f32, w2: f32, h2: f32) -> Option<RelativePosition> {
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

    if x1_max < x2_min {
        return Some(RelativePosition::Left);
    }

    if x1_min > x2_max {
        return Some(RelativePosition::Right);
    }

    if y1_max < y2_min {
        return Some(RelativePosition::Bottom);
    }

    if y1_min > y2_max {
        return Some(RelativePosition::Top);
    }

    None
}

pub fn equals(facing_direction: &EnnemyFacingDirection, relative_position: Option<RelativePosition>) -> bool {
    if let Some(relative_position) = relative_position {
        return equals_relative(facing_direction, relative_position);
    }
    false
}

pub fn get_ennemy_facing_from_player_facing(player_facing_direction: &PlayerFacingDirection) -> EnnemyFacingDirection {
    match player_facing_direction {
        PlayerFacingDirection::Up => EnnemyFacingDirection::Up,
        PlayerFacingDirection::Down => EnnemyFacingDirection::Down,
        PlayerFacingDirection::Left => EnnemyFacingDirection::Left,
        PlayerFacingDirection::Right => EnnemyFacingDirection::Right,
        PlayerFacingDirection::TopLeft => EnnemyFacingDirection::TopLeft,
        PlayerFacingDirection::TopRight => EnnemyFacingDirection::TopRight,
        PlayerFacingDirection::BottomLeft => EnnemyFacingDirection::BottomLeft,
        PlayerFacingDirection::BottomRight => EnnemyFacingDirection::BottomRight,
    }
}

fn equals_relative(facing_direction: &EnnemyFacingDirection, relative_position: RelativePosition) -> bool {
    match facing_direction {

        EnnemyFacingDirection::TopLeft => relative_position == RelativePosition::Top || relative_position == RelativePosition::Left,
        EnnemyFacingDirection::TopRight => relative_position == RelativePosition::Top || relative_position == RelativePosition::Right,
        EnnemyFacingDirection::BottomLeft => relative_position == RelativePosition::Bottom || relative_position == RelativePosition::Left,
        EnnemyFacingDirection::BottomRight => relative_position == RelativePosition::Bottom || relative_position == RelativePosition::Right,
        EnnemyFacingDirection::Up => relative_position == RelativePosition::Top,
        EnnemyFacingDirection::Down => relative_position == RelativePosition::Bottom,
        EnnemyFacingDirection::Left => relative_position == RelativePosition::Left,
        EnnemyFacingDirection::Right => relative_position == RelativePosition::Right,

    }
}
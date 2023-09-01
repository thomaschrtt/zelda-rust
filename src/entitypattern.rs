#[derive(PartialEq, Clone, Copy)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct EntityPatern {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    health: i32,
    facing_direction: Option<FacingDirection>,
}

pub trait EntityBehavior {
    fn attack(&mut self, target: &mut dyn EntityBehavior) -> bool;
    fn get_attacked(&mut self, damage: i32) -> bool;
    fn take_damage(&mut self, damage: i32) -> bool;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn add_x(&mut self, x: f32);
    fn add_y(&mut self, y: f32);

    fn facing_direction(&self) -> Option<FacingDirection>;
    fn set_facing_direction(&mut self, facing_direction: FacingDirection);
}

impl EntityPatern {
    pub fn new(x: f32, y: f32, w: f32, h: f32, health: i32) -> Self {
        EntityPatern { x, y, w, h, health, facing_direction: None }
    }

    pub fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn facing_direction(&self) -> Option<FacingDirection> {
        self.facing_direction
    }

    pub fn set_facing_direction(&mut self, facing_direction: FacingDirection) {
        self.facing_direction = Some(facing_direction);
    }

    pub fn add_x(&mut self, x: f32) {
        self.x += x
    }

    pub fn add_y(&mut self, y: f32) {
        self.y += y
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn add_health(&mut self, health: i32) {
        self.health += health;
    }

}

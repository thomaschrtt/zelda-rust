pub const SEED: u64 = 2;

// WINDOW
pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;
pub const MAP_SIZE: f32 = 1400.;
pub const CAMERA_DEFAULT_SCALE: f32 = 0.2;
pub const CAMERA_MIN_SCALE: f32 = 0.1;
pub const CAMERA_MAX_SCALE: f32 = 1.;

pub const Z_LAYER_BACKGROUND: f32 = 0.;
pub const Z_LAYER_PLAYER: f32 = 1.;
pub const Z_LAYER_GUI: f32 = f32::MAX;
pub const Z_LAYER_STRUCTURES: f32 = 2.;
pub const Z_LAYER_ENNEMIES: f32 = Z_LAYER_PLAYER;

// PLAYER
pub const PLAYER_SPRITE_SIZE: f32 = 32.;
pub const PLAYER_SPRITE_SCALE: f32 = 0.5;
pub const PLAYER_HITBOX_WIDTH: f32 = PLAYER_SPRITE_SIZE * PLAYER_SPRITE_SCALE;
pub const PLAYER_HITBOX_HEIGHT: f32 = PLAYER_SPRITE_SIZE * PLAYER_SPRITE_SCALE;
pub const PLAYER_NORMAL_SPEED: f32 = 1.;
pub const PLAYER_SPRINT_SPEED: f32 = 2.;
pub const PLAYER_DAMAGE: i32 = 2;
pub const PLAYER_ATTACK_RANGE: f32 = 5.;
pub const PLAYER_ATTACK_DELAY: u64 = 1000;
pub const PLAYER_HEALTH: i32 = 20;

// STRUCTURES
pub const TOWER_HEIGHT: f32 = 128.;
pub const TOWER_WIDTH: f32 = 67.;
pub const SANCTUARY_HEIGHT: f32 = 75.;
pub const SANCTUARY_WIDTH: f32 = 96.;
pub const SANCTUARY_NB: i32 = 3;
pub const SANCTUARY_HEALING: i32 = 5;

// ENNEMIES
pub const ENNEMIES_NUMBER: i32 = 45;
pub const ENNEMY_SPRITE_SIZE: f32 = 32.;
pub const ENNEMY_SPRITE_SCALE: f32 = 0.5;
pub const ENNEMY_HITBOX_WIDTH: f32 = ENNEMY_SPRITE_SIZE * ENNEMY_SPRITE_SCALE;
pub const ENNEMY_HITBOX_HEIGHT: f32 = ENNEMY_SPRITE_SIZE * ENNEMY_SPRITE_SCALE;
pub const ENNEMY_SPRINT_SPEED: f32 = 1.2;
pub const ENNEMY_NORMAL_SPEED: f32 = 0.6;
pub const ENNEMY_ATTACK_SPEED: f32 = 0.4;
pub const ENNEMY_ATTACK_RANGE: f32 = 5.;
pub const ENNEMY_AGGRO_DISTANCE: f32 = 120.;

// OTHERS
pub const TREE_HEIGHT: f32 = 160.;
pub const TREE_WIDTH: f32 = 128.;
pub const TREE_TRANSPARENCY: f32 = 0.6;
pub const TREE_NUMBER: i32 = 100;
pub const BUSH_HEIGHT: f32 = 50.;
pub const BUSH_WIDTH: f32 = 55.;
pub const BUSH_TRANSPARENCY: f32 = 0.6;
pub const BUSH_NUMBER: i32 = 100;
pub const GRAVES_NUMBER: i32 = 25;


// RNG OFFSETS
pub const OFFSET_TREE: u64 = 0;
pub const OFFSET_BUSH: u64 = 1000;
pub const OFFSET_GRAVE: u64 = 2000;
pub const OFFSET_ENNEMY: u64 = 3000;
pub const OFFSET_SANCTUARY: u64 = 4000;
pub const OFFSET_TOWER: u64 = 5000;



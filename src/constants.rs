// WINDOW
pub const WINDOW_SIZE: f32 = 600.;
pub const MAP_SIZE: f32 = 1400.;
pub const CAMERA_DEFAULT_SCALE: f32 = 0.3;
pub const CAMERA_DEFAULT_SIZE: f32 = WINDOW_SIZE * CAMERA_DEFAULT_SCALE;
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
pub const PLAYER_ATTACK_DELAY: u64 = 1;

// STRUCTURES
pub const TOWER_HEIGHT: f32 = 128.;
pub const TOWER_WIDTH: f32 = 67.;
pub const SANCTUARY_HEIGHT: f32 = 75.;
pub const SANCTUARY_WIDTH: f32 = 96.;
pub const SANCTUARY_NB: i32 = 10;

// ENNEMIES
pub const ENNEMY_SPRITE_SIZE: f32 = 32.;
pub const ENNEMY_SPRITE_SCALE: f32 = 0.5;
pub const ENNEMY_HITBOX_WIDTH: f32 = ENNEMY_SPRITE_SIZE * ENNEMY_SPRITE_SCALE;
pub const ENNEMY_HITBOX_HEIGHT: f32 = ENNEMY_SPRITE_SIZE * ENNEMY_SPRITE_SCALE;
pub const ENNEMY_SPRINT_SPEED: f32 = 1.2;
pub const ENNEMY_NORMAL_SPEED: f32 = 0.6;
pub const ENNEMY_ATTACK_DELAY: u64 = 2;
pub const ENNEMY_ATTACK_RANGE: f32 = 5.;
pub const ENNEMY_AGGRO_DISTANCE: f32 = 75.;

// OTHERS
pub const TREE_HEIGHT: f32 = 140.;
pub const TREE_WIDTH: f32 = 115.;
pub const BUSH_HEIGHT: f32 = 50.;
pub const BUSH_WIDTH: f32 = 55.;
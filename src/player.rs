use bevy::prelude::*;
use crate::constants::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_move, player_facing_direction, update_player_pos));
    }
}

#[derive(Component)]
pub struct Player {
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}


fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let left_boundary = (-((WINDOW_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.)) / PLAYER_SPEED) as i32;
    let right_boundary = -left_boundary;
    let top_boundary = right_boundary;
    let bottom_boundary = left_boundary;

    let mut player = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) && player.x > left_boundary {
        player.x -= 1;
    }
    if keyboard_input.pressed(KeyCode::Right) && player.x < right_boundary {
        player.x += 1;
    }
    if keyboard_input.pressed(KeyCode::Up) && player.y < top_boundary {
        player.y += 1;
    }
    if keyboard_input.pressed(KeyCode::Down) && player.y > bottom_boundary {
        player.y -= 1;
    }
}

fn player_facing_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        player.scale.x = -PLAYER_SPRITE_SCALE;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player.scale.x = PLAYER_SPRITE_SCALE;
    }
}

fn update_player_pos(
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let mut player = query.single_mut();
    player.1.translation.x = player.0.x as f32 * PLAYER_SPEED;
    player.1.translation.y = player.0.y as f32 * PLAYER_SPEED;
}
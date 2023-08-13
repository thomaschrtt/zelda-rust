use bevy::prelude::*;
use crate::constants::*;
use crate::collisions::*;
use crate::structures;
use crate::structures::*;

enum PlayerFacingDirection {
    Left,
    Up,
    Right,
    Down,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_move, 
                                                    update_player_pos, 
                                                    player_facing_direction, 
                                                    update_player_sprite_moving,
                                                    tower_detection,
                                                    sanctuary_detection));
    }
}


#[derive(Component)]
pub struct Player {
    x: i32,
    y: i32,
    facing_direction: PlayerFacingDirection,
    sprinting: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0, y: 0, facing_direction: PlayerFacingDirection::Right, sprinting:false }
    }
}

impl Collisionable for Player {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, (PLAYER_HITBOX_WIDTH * 0.8) as i32, (PLAYER_HITBOX_HEIGHT*0.8) as i32)
    }
    
}


fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    collisionable_query: Query<&CollisionComponent>,
) {

    let player_speed: i32;
    let mut player = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        player.sprinting = true;
        player_speed = 2;

    } else {
        player.sprinting = false;
        player_speed = 1;
    }


    let left_boundary = -((MAP_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.)) as i32;
    let right_boundary = -left_boundary;
    let top_boundary = right_boundary;
    let bottom_boundary = left_boundary;

    let new_x = if keyboard_input.pressed(KeyCode::Left) { player.x - player_speed }
                     else if keyboard_input.pressed(KeyCode::Right) { player.x + player_speed }
                     else { player.x };

    let new_y = if keyboard_input.pressed(KeyCode::Down) { player.y - player_speed }
                     else if keyboard_input.pressed(KeyCode::Up) { player.y + player_speed }
                     else { player.y };


    for collidable in collisionable_query.iter() {
        if player.would_collide(new_x, new_y, collidable){ 
            return;  
        }
    }
    if new_x < left_boundary || new_x > right_boundary || new_y < bottom_boundary || new_y > top_boundary {
        return;
    }

    player.x = new_x;
    player.y = new_y;
}
    

fn player_facing_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        player.facing_direction = PlayerFacingDirection::Left;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player.facing_direction = PlayerFacingDirection::Right;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::Up;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::Down;
    }
}   


fn update_player_sprite_moving(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut TextureAtlasSprite)>,
) {
    let mut player = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Down) {
        if player.0.sprinting {
            player.1.index = 24;
        }
        else {
            player.1.index = 26;
        }
    }
    else {
        player.1.index = 0;
    }
}

fn update_player_pos(
    mut query: Query<(&mut Player, &mut Transform)>,
) {

    let (player, mut sprite) = query.single_mut();
    let x = player.x as f32;
    let y = player.y as f32;
    sprite.translation.x = x;
    sprite.translation.y = y;
    match player.facing_direction {
        PlayerFacingDirection::Left => sprite.scale.x = -PLAYER_SPRITE_SCALE,
        PlayerFacingDirection::Right => sprite.scale.x = PLAYER_SPRITE_SCALE,
        _ => {}
    }
}

fn spawn_player(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) 
    {

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE), 8, 9, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player: Player = Player::new();

    commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, Z_LAYER_PLAYER),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(player);
}
enum InteractionType {
    Tower,
    Sanctuary,
}
fn can_interact_with(
    player: &Player,
    interaction_type: &InteractionType,
    x: i32,
    y: i32,
    tower: Option<&Tower>,
    sanctuary: Option<&Sanctuary>,
) -> bool {
    match interaction_type {
        InteractionType::Tower => {
            if let Some(t) = tower {
                player.would_collide(x, y, &CollisionComponent::new_from_component(t))
            } else {
                false
            }
        }
        InteractionType::Sanctuary => {
            if let Some(s) = sanctuary {
                player.would_collide(x, y, &CollisionComponent::new_from_component(s))
            } else {
                false
            }
        }
    }
}

fn tower_detection(
    mut player_query: Query<&mut Player>,
    tower_query: Query<&Tower>,
    keyboard_input: Res<Input<KeyCode>>,
    query_sanctuary: Query<&mut Sanctuary>,
) {
    let player = player_query.single_mut();
    for tower in tower_query.iter() {
        if can_interact_with(&player, &InteractionType::Tower, player.x, player.y + 1, Some(tower), None) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                structures::show_one_sanctuary(query_sanctuary);
                break;
            }
        }
    }
}

fn sanctuary_detection(
    mut player_query: Query<&mut Player>,
    mut sanctuary_query: Query<&mut Sanctuary>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player = player_query.single_mut();
    for mut sanctuary in sanctuary_query.iter_mut() {
        if can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y + 1, None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y - 1, None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x + 1, player.y, None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x - 1, player.y, None, Some(&sanctuary)) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                sanctuary.unlock();
                break;
            }
        }
    }
}


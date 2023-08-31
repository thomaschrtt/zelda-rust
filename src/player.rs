use bevy::prelude::*;
use crate::collisions;
use crate::constants::*;
use crate::collisions::*;
use crate::ennemies;
use crate::ennemies::*;
use crate::structures;
use crate::structures::*;
use crate::setup::*;

enum InteractionType {
    Tower,
    Sanctuary,
    Ennemy,
}

#[derive(PartialEq)]
pub enum PlayerFacingDirection {
    Left,
    TopLeft,
    Up,
    TopRight,
    Right,
    BottomRight,
    Down,
    BottomLeft,
}

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Moving,
    Sprinting,
    Attacking,
    Blocking,
    Dead
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_move, 
                                                    update_player_pos, 
                                                    player_facing_direction, 
                                                    update_player_sprite,
                                                    tower_detection,
                                                    sanctuary_detection,
                                                    tree_transparency,
                                                    update_hitbox_pos,
                                                    update_hitbox_visibility,
                                                    ennemy_detection,
                                                    update_collisionable_pos,
                                                    update_player_state,
                                                    update_dead_player));
    }
}


#[derive(Component)]
pub struct Player {
    x: f32,
    y: f32,
    facing_direction: PlayerFacingDirection,
    state: PlayerState,
    health: i32,

    idle_frame_counter: usize,
    idle_frame_time: f32,

    attack_frame_counter: usize,
    attack_frame_time: f32,

    sprint_frame_counter: usize,
    sprint_frame_time: f32,

    walk_frame_counter: usize,
    walk_frame_time: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0., y: 0., facing_direction: PlayerFacingDirection::Right, state: PlayerState::Idle, health: 20, idle_frame_counter: 0, idle_frame_time: 0., attack_frame_counter: 0, attack_frame_time: 0., sprint_frame_counter: 0, sprint_frame_time: 0., walk_frame_counter: 0, walk_frame_time: 0. }
    }

    fn is_facing_down(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Down || self.facing_direction == PlayerFacingDirection::BottomLeft || self.facing_direction == PlayerFacingDirection::BottomRight
    }

    fn is_facing_up(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Up || self.facing_direction == PlayerFacingDirection::TopLeft || self.facing_direction == PlayerFacingDirection::TopRight
    }

    fn is_facing_left(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Left || self.facing_direction == PlayerFacingDirection::TopLeft || self.facing_direction == PlayerFacingDirection::BottomLeft
    }

    fn is_facing_right(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Right || self.facing_direction == PlayerFacingDirection::TopRight || self.facing_direction == PlayerFacingDirection::BottomRight
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn get_attacked(&mut self, damage: i32) -> bool {
        if self.is_blocking() {
            println!("Player blocked the attack");
            return false;
        }
        self.take_damage(damage);
        println!("Player took {} damage", damage);
        true
    }

    fn attack(&mut self, ennemy: &mut Ennemy) -> bool {
        return ennemy.get_attacked(PLAYER_DAMAGE);
    }

    fn is_blocking(&self) -> bool {
        self.state == PlayerState::Blocking
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }
}

impl Collisionable for Player {
    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, PLAYER_HITBOX_WIDTH * 0.8, PLAYER_HITBOX_HEIGHT*0.8)
    }
    
}


#[derive(Component)]
pub struct HitBox;

fn spawn_player(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) 
    {

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE), 8, 9, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 1.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player: Player = Player::new();
    let collisioncomponent = CollisionComponent::new(player.x, player.y, PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT);

    let attack_delay = ennemies::AttackDelay::new(PLAYER_ATTACK_DELAY);
    
    let hitbox = player.get_hitbox();

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
        .insert(player)
        .insert(attack_delay)
        .insert(collisioncomponent);

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., Z_LAYER_GUI),
            ..Transform::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(hitbox.2, hitbox.3)),
            color: Color::rgb(0.0, 0.0, 1.0),
            ..Default::default()
        },
        visibility: Visibility::Hidden,
        ..Default::default()
    }).insert(HitBox);
}

fn update_player_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();

    if player.state == PlayerState::Attacking || player.state == PlayerState::Dead {
        return;
    }

    // CLASSE PAR ORDRE DIMPORTANCE

    if keyboard_input.pressed(KeyCode::ShiftLeft){
        player.state = PlayerState::Sprinting;
    }
    
    else if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right) {
        player.state = PlayerState::Moving;
    } 
    else if keyboard_input.pressed(KeyCode::E) {
        player.state = PlayerState::Blocking;
    }
    else {
        player.state = PlayerState::Idle;
    }
}

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    collisionable_query: Query<&CollisionComponent, Without<Player>>,
) {
    
    let player_speed: f32;
    let mut player = player_query.single_mut();

    if player.is_dead() {
        return;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        player_speed = PLAYER_SPRINT_SPEED;

    } else {
        player_speed = PLAYER_NORMAL_SPEED;
    }
    let left_boundary = -((MAP_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.));
    let right_boundary = -left_boundary;
    let top_boundary = right_boundary;
    let bottom_boundary = left_boundary;

    let actual_x = player.x;
    let actual_y = player.y;

    let mut new_x = if keyboard_input.pressed(KeyCode::Left) { player.x - player_speed }
                     else if keyboard_input.pressed(KeyCode::Right) { player.x + player_speed }
                     else { player.x };

    for collidable in collisionable_query.iter() {
        if player.would_collide(new_x, player.y, collidable){ 
            new_x = actual_x;
        }
    }

    if new_x > right_boundary {
        new_x = actual_x;
    } else if new_x < left_boundary {
        new_x = actual_x;
    }

    let mut new_y = if keyboard_input.pressed(KeyCode::Down) { player.y - player_speed }
                     else if keyboard_input.pressed(KeyCode::Up) { player.y + player_speed }
                     else { player.y };

    for collidable in collisionable_query.iter() {
        if player.would_collide(player.x, new_y, collidable){ 
            new_y = actual_y;
        }
    }

    if new_y > top_boundary {
        new_y = actual_y;
    } else if new_y < bottom_boundary {
        new_y = actual_y;
    }
    player.x = new_x;
    player.y = new_y;
}


fn player_facing_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();

    if player.is_dead() {
        return;
    }

    if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::TopLeft;
    }
    else if keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::TopRight;
    }
    else if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::BottomLeft;
    }
    else if keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::BottomRight;
    }
    else if keyboard_input.pressed(KeyCode::Left) {
        player.facing_direction = PlayerFacingDirection::Left;
    }
    else if keyboard_input.pressed(KeyCode::Right) {
        player.facing_direction = PlayerFacingDirection::Right;
    }
    else if keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::Up;
    }
    else if keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::Down;
    }
}


fn update_player_sprite(
    mut query: Query<(&mut Player, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    let (mut player, mut texture) = query.single_mut();
    match player.state {
        PlayerState::Idle => {
            player.idle_frame_time += time.delta_seconds();

            if player.idle_frame_time >= 0.4 {
                player.idle_frame_counter = (player.idle_frame_counter + 1) % 4;
                player.idle_frame_time = 0.0; 
            }
            texture.index = match player.idle_frame_counter {
                0 => 0,
                1 => 1,
                2 => 8,
                3 => 9,
                _ => 0,
            };
        },
        PlayerState::Blocking => texture.index = 2,
        PlayerState::Moving => {
            player.walk_frame_time += time.delta_seconds();

            if player.walk_frame_time >= 0.2 {
                player.walk_frame_counter = (player.walk_frame_counter + 1) % 4;
                player.walk_frame_time = 0.0; 
            }
            texture.index = 16 + player.walk_frame_counter;
        },
        PlayerState::Sprinting => {
            player.sprint_frame_time += time.delta_seconds();

            if player.sprint_frame_time >= 0.1 {
                player.sprint_frame_counter = (player.sprint_frame_counter + 1) % 8;
                player.sprint_frame_time = 0.0; 
            }
            texture.index = 24 + player.sprint_frame_counter;

        },
        PlayerState::Attacking => {
            player.attack_frame_time += time.delta_seconds();

            // Changer de frame toutes les 0.1 secondes (ou selon votre choix)
            if player.attack_frame_time >= 0.1 {
                player.attack_frame_counter = (player.attack_frame_counter + 1) % 8;
                player.attack_frame_time = 0.0; // Réinitialiser le temps écoulé
            }

            // Mettre à jour l'index de la texture
            texture.index = 64 + player.attack_frame_counter;
        },
        PlayerState::Dead => {
            texture.index = 63;
        },
    }
}

fn update_dead_player(
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();
    if player.health <= 0 {
        player.state = PlayerState::Dead;
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
        PlayerFacingDirection::TopLeft => sprite.scale.x = -PLAYER_SPRITE_SCALE,
        PlayerFacingDirection::BottomLeft => sprite.scale.x = -PLAYER_SPRITE_SCALE,
        PlayerFacingDirection::Right => sprite.scale.x = PLAYER_SPRITE_SCALE,
        PlayerFacingDirection::TopRight => sprite.scale.x = PLAYER_SPRITE_SCALE,
        PlayerFacingDirection::BottomRight => sprite.scale.x = PLAYER_SPRITE_SCALE,
        _ => {}
    }
}

fn update_hitbox_pos(
    player_query: Query<&Player>,
    mut hitbox_query: Query<&mut Transform, With<HitBox>>,
) {
    let player = player_query.single();
    for mut transform in hitbox_query.iter_mut() {
        transform.translation.x = player.x;
        transform.translation.y = player.y;
    }
}

fn update_hitbox_visibility(
    keyboard_input: Res<Input<KeyCode>>,
    mut hitbox_query: Query<&mut Visibility, With<HitBox>>,
) {
    for mut visibility in hitbox_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::L) {
            *visibility = Visibility::Visible;
        }
        if keyboard_input.just_pressed(KeyCode::K) {
            *visibility = Visibility::Hidden;
        }
    }
}

fn update_collisionable_pos(
    mut query: Query<(&mut CollisionComponent, &Player)>,
) {
    let (mut collisionable, player) = query.single_mut();
    collisionable.set_pos(player.x, player.y)
}

fn can_interact_with(
    player: &Player,
    interaction_type: &InteractionType,
    x: f32,
    y: f32,
    tower: Option<&Tower>,
    sanctuary: Option<&Sanctuary>,
    ennemy: Option<&Ennemy>,
) -> bool {

    if player.is_dead() {
        return false;
    }

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
        InteractionType::Ennemy => {
            if let Some(e) = ennemy {
                player.would_collide(x, y, &CollisionComponent::new_from_component(e))
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
        if can_interact_with(&player, &InteractionType::Tower, player.x, player.y + 1., Some(tower), None, None) {
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
        if can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y + 1., None, Some(&sanctuary), None) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y - 1., None, Some(&sanctuary), None) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x + 1., player.y, None, Some(&sanctuary), None) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x - 1., player.y, None, Some(&sanctuary), None) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                sanctuary.unlock();
                break;
            }
        }
    }
}

fn ennemy_detection(
    mut player_query: Query<(&mut Player, &mut AttackDelay)>,
    mut ennemy_query: Query<&mut Ennemy>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut attack_delay) = player_query.single_mut();
    
    attack_delay.timer.tick(time.delta());


    if attack_delay.timer.finished() {
        if player.state == PlayerState::Attacking {
            player.state = PlayerState::Idle;
        }
        for mut ennemy in ennemy_query.iter_mut() {
            if can_interact_with(&player, &InteractionType::Ennemy, player.x, player.y + PLAYER_ATTACK_RANGE, None, None, Some(&ennemy)) ||
                can_interact_with(&player, &InteractionType::Ennemy, player.x, player.y - PLAYER_ATTACK_RANGE, None, None, Some(&ennemy)) ||
                can_interact_with(&player, &InteractionType::Ennemy, player.x + PLAYER_ATTACK_RANGE, player.y, None, None, Some(&ennemy)) ||
                can_interact_with(&player, &InteractionType::Ennemy, player.x - PLAYER_ATTACK_RANGE, player.y, None, None, Some(&ennemy)) {
                if keyboard_input.just_pressed(KeyCode::Space) {
                    player.attack(&mut ennemy);
                    player.state = PlayerState::Attacking;
                    attack_delay.timer.reset();
                    break;
                }
            }
        }
    }
}

fn tree_transparency(
    player_query: Query<&Player>,
    mut tree_query: Query<(&mut TextureAtlasSprite, &Transform), With<Tree>>,
) {
    let player = player_query.single();
    for (mut sprite, transform) in tree_query.iter_mut() {
        if collisions::are_overlapping(player.x, player.y, PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT, transform.translation.x, transform.translation.y, TREE_WIDTH*0.6, TREE_HEIGHT*0.6) {
            sprite.color.set_a(0.50);
        } else {
            sprite.color.set_a(1.0);
        }
    }
}


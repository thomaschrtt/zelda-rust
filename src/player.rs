use std::time::Duration;

use bevy::prelude::*;
use crate::GameConfig;
use crate::collisions;
use crate::constants::*;
use crate::collisions::*;
use crate::ennemies;
use crate::ennemies::*;
use crate::entitypattern::*;
use crate::structures;
use crate::structures::*;
use crate::setup::*;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Menu), spawn_player)
            .add_systems(Update, (player_move, 
                                                    update_player_pos, 
                                                    player_facing_direction, 
                                                    update_player_sprite,
                                                    tower_detection,
                                                    sanctuary_detection,
                                                    background_elements_transparency,
                                                    update_hitbox_pos,
                                                    // update_hitbox_visibility,
                                                    ennemy_detection,
                                                    update_collision,
                                                    update_player_state,
                                                    slide_out_of_collision,
                                                    switch_to_game_over
                                                ).run_if(in_state(GameState::Playing)));
    }
}


#[derive(Component)]
pub struct AttackDelay {
    pub timer: Timer,
}

impl AttackDelay {
    pub fn new(delay: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(delay), TimerMode::Once),
        }
    }
}


enum InteractionType {
    Tower,
    Sanctuary,
    Ennemy,
}

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Moving,
    Sprinting,
    Attacking,
    Blocking,
    Damaged,
    Healing,
    Dying,
    Dead
}


#[derive(Component)]
pub struct Player {
    self_entity: EntityPatern,
    state: PlayerState,

    idle_frame_counter: usize,
    idle_frame_time: f32,

    attack_frame_counter: usize,
    attack_frame_time: f32,

    sprint_frame_counter: usize,
    sprint_frame_time: f32,

    walk_frame_counter: usize,
    walk_frame_time: f32,

    healing_frame_counter: usize,
    healing_frame_time: f32,
    healing_duration_elapsed: f32,

    damaged_frame_counter: usize,
    damaged_frame_time: f32,
    damaged_duration_elapsed: f32,

    dying_frame_counter: usize,
    dying_frame_time: f32,
    dying_duration_elapsed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { 
               self_entity: EntityPatern::new(0., 0., PLAYER_HITBOX_WIDTH * 0.8, PLAYER_HITBOX_HEIGHT * 0.8, PLAYER_HEALTH),
               state: PlayerState::Idle, 
               idle_frame_counter: 0, idle_frame_time: 0., 
               attack_frame_counter: 0, attack_frame_time: 0., 
               sprint_frame_counter: 0, sprint_frame_time: 0., 
               walk_frame_counter: 0, walk_frame_time: 0.,
               healing_frame_counter: 0, healing_frame_time: 0., healing_duration_elapsed: 0.,
               damaged_frame_counter: 0, damaged_frame_time: 0., damaged_duration_elapsed: 0.,
                dying_frame_counter: 0, dying_frame_time: 0., dying_duration_elapsed: 0.,
             }
    }

    pub fn is_aggroable(&self) -> bool {
        if self.is_dead() || self.is_healing() || self.is_dying() {
            return false;
        }
        true
    }

    fn can_move(&self) -> bool {
        if self.is_dead() || self.is_healing() || self.is_dying() {
            return false;
        }
        true
    }

    fn can_change_facing_direction(&self) -> bool {
        if self.is_dead() || self.is_healing() {
            return false;
        }
        true
    }

    fn can_interact(&self) -> bool {
        if self.is_dead() || self.is_healing() || self.is_dying() {
            return false;
        }
        true
    }

    fn is_attacking(&self) -> bool {
        self.state == PlayerState::Attacking
    }

    fn is_blocking(&self) -> bool {
        self.state == PlayerState::Blocking
    }

    fn is_damaged(&self) -> bool {
        self.state == PlayerState::Damaged
    }

    fn is_healing(&self) -> bool {
        self.state == PlayerState::Healing
    }

    fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    fn is_dying(&self) -> bool {
        self.state == PlayerState::Dying
    }

    fn heal(&mut self) {
        self.self_entity.add_health(SANCTUARY_HEALING);
        println!("Player health now at {}", self.self_entity.health());
        self.state = PlayerState::Healing;

    }
}

impl Collisionable for Player {
    fn get_pos(&self) -> (f32, f32) {
        self.self_entity.get_pos()
    }
    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        self.self_entity.get_hitbox()
    }
    
}

impl EntityBehavior for Player {
    fn attack(&mut self, target: &mut dyn EntityBehavior) -> bool {
        return target.get_attacked(PLAYER_DAMAGE);
    }

    fn get_attacked(&mut self, damage: i32) -> bool {
        if !self.is_aggroable() {
            return false;
        }
        if self.is_blocking() {
            println!("Player blocked the attack");
            return false;
        }
        self.take_damage(damage);
        println!("Player took {} damage", damage);
        true
    }

    fn take_damage(&mut self, damage: i32) -> bool {
        self.self_entity.add_health(-damage);
        self.state = PlayerState::Damaged;
        if self.self_entity.health() <= 0 {
            self.state = PlayerState::Dying;
        }
        true
    }

    fn x(&self) -> f32 {
        self.self_entity.x()
    }

    fn y(&self) -> f32 {
        self.self_entity.y()
    }

    fn set_x(&mut self, x: f32) {
        self.self_entity.set_x(x);
    }

    fn set_y(&mut self, y: f32) {
        self.self_entity.set_y(y);
    }

    fn facing_direction(&self) -> Option<FacingDirection> {
        self.self_entity.facing_direction()
    }

    fn set_facing_direction(&mut self, facing_direction: FacingDirection) {
        self.self_entity.set_facing_direction(facing_direction);
    }

    fn add_x(&mut self, x: f32) {
        self.self_entity.add_x(x);
    }

    fn add_y(&mut self, y: f32) {
        self.self_entity.add_y(y);
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
    let (x, y) = player.get_pos();
    let collisioncomponent = CollisionComponent::new(x, y, PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT);

    let attack_delay = AttackDelay::new(PLAYER_ATTACK_DELAY);
    
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

    if player.is_attacking() || player.is_dead() || player.is_healing() || player.is_damaged() || player.is_dying() {
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

    if !player.can_move() {
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

    let actual_x = player.x();
    let actual_y = player.y();

    let mut new_x = if keyboard_input.pressed(KeyCode::Left) { player.x() - player_speed }
                     else if keyboard_input.pressed(KeyCode::Right) { player.x() + player_speed }
                     else { player.x() };

    for collidable in collisionable_query.iter() {
        if player.would_collide(new_x, player.y(), collidable){ 
            new_x = actual_x;
        }
    }

    if new_x > right_boundary {
        new_x = actual_x;
    } else if new_x < left_boundary {
        new_x = actual_x;
    }

    let mut new_y = if keyboard_input.pressed(KeyCode::Down) { player.y() - player_speed }
                     else if keyboard_input.pressed(KeyCode::Up) { player.y() + player_speed }
                     else { player.y() };

    for collidable in collisionable_query.iter() {
        if player.would_collide(player.x(), new_y, collidable){ 
            new_y = actual_y;
        }
    }

    if new_y > top_boundary {
        new_y = actual_y;
    } else if new_y < bottom_boundary {
        new_y = actual_y;
    }
    player.set_x(new_x);
    player.set_y(new_y);
}


fn player_facing_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();

    if !player.can_change_facing_direction() {
        return;
    }
    if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Up) {
        player.set_facing_direction(FacingDirection::TopLeft)
    }
    else if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Down) {
        player.set_facing_direction(FacingDirection::BottomLeft)
    }
    else if keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Up) {
        player.set_facing_direction(FacingDirection::TopRight)
    }
    else if keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Down) {
        player.set_facing_direction(FacingDirection::BottomRight)
    }
    else if keyboard_input.pressed(KeyCode::Left) {
        player.set_facing_direction(FacingDirection::Left)
    }
    else if keyboard_input.pressed(KeyCode::Right) {
        player.set_facing_direction(FacingDirection::Right)
    }
    else if keyboard_input.pressed(KeyCode::Up) {
        player.set_facing_direction(FacingDirection::Up)
    }
    else if keyboard_input.pressed(KeyCode::Down) {
        player.set_facing_direction(FacingDirection::Down)
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
                player.attack_frame_counter = player.attack_frame_counter + 1;
                player.attack_frame_time = 0.0; // Réinitialiser le temps écoulé
            }

            // Mettre à jour l'index de la texture
            texture.index = 64 + player.attack_frame_counter;
            if player.attack_frame_counter >= 7 {
                player.state = PlayerState::Idle;
                player.attack_frame_counter = 0;
            }
        },

        PlayerState::Dying => {
            player.dying_frame_time += time.delta_seconds();
            player.dying_duration_elapsed += time.delta_seconds();

            if player.dying_frame_time >= 1./8. {
                player.dying_frame_counter = (player.dying_frame_counter + 1) % 8;
                player.dying_frame_time = 0.0; 
            }
            texture.index = 56 + player.dying_frame_counter;
            if player.dying_duration_elapsed >= 1. {
                player.state = PlayerState::Dead;
                player.dying_duration_elapsed = 0.;
            }
        },
        
        PlayerState::Dead => {
            texture.index = 63;
        },
        PlayerState::Healing => {
            player.healing_frame_time += time.delta_seconds();
            player.healing_duration_elapsed += time.delta_seconds();

            if player.healing_frame_time >= 0.4 {
                player.healing_frame_counter = (player.healing_frame_counter + 1) % 3;
                player.healing_frame_time = 0.0; 
            }
            texture.index = 3 + player.healing_frame_counter;
            if player.healing_duration_elapsed >= 3. {
                player.state = PlayerState::Idle;
                player.healing_duration_elapsed = 0.;
            }
            
        },
        PlayerState::Damaged => {
            player.damaged_frame_time += time.delta_seconds();
            player.damaged_duration_elapsed += time.delta_seconds();

            if player.damaged_frame_time >= 0.15 {
                player.damaged_frame_counter = (player.damaged_frame_counter + 1) % 3;
                player.damaged_frame_time = 0.0; 
            }
            texture.index = 48 + player.damaged_frame_counter;
            if player.damaged_duration_elapsed >= 0.15*3. {
                player.state = PlayerState::Idle;
                player.damaged_duration_elapsed = 0.;
            }
        }
    }
}

fn switch_to_game_over(
    mut nextstate : ResMut<NextState<GameState>>,
    player_query: Query<&Player>
) {
    let player = player_query.single();
    if player.is_dead() {
        nextstate.set(GameState::GameOver);
    }
}

fn update_player_pos(
    mut query: Query<(&mut Player, &mut Transform)>,
) {

    let (player, mut sprite) = query.single_mut();
    let x = player.x();
    let y = player.y();
    sprite.translation.x = x;
    sprite.translation.y = y;
    if let Some(facing_direction) = &player.facing_direction() {
        match facing_direction {
            FacingDirection::Left => sprite.scale.x = -PLAYER_SPRITE_SCALE,
            FacingDirection::TopLeft => sprite.scale.x = -PLAYER_SPRITE_SCALE,
            FacingDirection::BottomLeft => sprite.scale.x = -PLAYER_SPRITE_SCALE,
            FacingDirection::Right => sprite.scale.x = PLAYER_SPRITE_SCALE,
            FacingDirection::TopRight => sprite.scale.x = PLAYER_SPRITE_SCALE,
            FacingDirection::BottomRight => sprite.scale.x = PLAYER_SPRITE_SCALE,
            _ => {}
        }
    }
}

fn update_hitbox_pos(
    player_query: Query<&Player>,
    mut hitbox_query: Query<&mut Transform, With<HitBox>>,
) {
    let player = player_query.single();
    for mut transform in hitbox_query.iter_mut() {
        transform.translation.x = player.x();
        transform.translation.y = player.y();
    }
}

// fn update_hitbox_visibility(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut hitbox_query: Query<&mut Visibility, With<HitBox>>,
// ) {
//     for mut visibility in hitbox_query.iter_mut() {
//         if keyboard_input.just_pressed(KeyCode::L) {
//             *visibility = Visibility::Visible;
//         }
//         if keyboard_input.just_pressed(KeyCode::K) {
//             *visibility = Visibility::Hidden;
//         }
//     }
// }

fn update_collision(
    mut query: Query<(&mut CollisionComponent, &Player)>,
) {
    update_collisionable_pos::<Player>(&mut query);
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

    if !player.can_interact() {
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
    game_config: Res<GameConfig>,
    nextstate : ResMut<NextState<GameState>>
) {
    let player = player_query.single_mut();
    for tower in tower_query.iter() {
        if can_interact_with(&player, &InteractionType::Tower, player.x(), player.y() + 1., Some(tower), None, None) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                structures::show_one_sanctuary(query_sanctuary, game_config, nextstate);
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
    let mut player = player_query.single_mut();
    for mut sanctuary in sanctuary_query.iter_mut() {
        if can_interact_with(&player, &InteractionType::Sanctuary, player.x(), player.y() + 1., None, Some(&sanctuary), None) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x(), player.y() - 1., None, Some(&sanctuary), None) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x() + 1., player.y(), None, Some(&sanctuary), None) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x() - 1., player.y(), None, Some(&sanctuary), None) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                if sanctuary.unlock() {  
                    player.heal();
                }
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
        if player.is_attacking() {
            player.state = PlayerState::Idle;
        }
        for mut ennemy in ennemy_query.iter_mut() {
            if can_interact_with(&player, &InteractionType::Ennemy, player.x(), player.y() + PLAYER_ATTACK_RANGE, None, None, Some(&ennemy)) ||
                can_interact_with(&player, &InteractionType::Ennemy, player.x(), player.y() - PLAYER_ATTACK_RANGE, None, None, Some(&ennemy)) ||
                can_interact_with(&player, &InteractionType::Ennemy, player.x() + PLAYER_ATTACK_RANGE, player.y(), None, None, Some(&ennemy)) ||
                can_interact_with(&player, &InteractionType::Ennemy, player.x() - PLAYER_ATTACK_RANGE, player.y(), None, None, Some(&ennemy)) {
                if keyboard_input.just_pressed(KeyCode::Space) {
                    let actual_ennemy: &mut ennemies::Ennemy = &mut ennemy;
                    player.attack(actual_ennemy);
                    player.state = PlayerState::Attacking;
                    attack_delay.timer.reset();
                    break;
                }
            }
        }
    }
}

pub fn background_elements_transparency(
    mut player_query: Query<&mut Player>,
    mut background_objects: Query<(&mut TextureAtlasSprite, &Transform, &BackgroundObjects)>,
) {
    let player = player_query.single_mut();
    for (mut sprite, transform, obj) in background_objects.iter_mut() {
        let (x, y) = player.get_pos();
        let (bg_obj_width, bg_obj_height) = (
            match obj.get_type() {
            BackgroundObjectType::Tree => TREE_WIDTH*TREE_TRANSPARENCY,
            BackgroundObjectType::Bush => BUSH_WIDTH*BUSH_TRANSPARENCY,
            _ => 0.,
        }, match obj.get_type() {
            BackgroundObjectType::Tree => TREE_HEIGHT*TREE_TRANSPARENCY,
            BackgroundObjectType::Bush => BUSH_HEIGHT*BUSH_TRANSPARENCY,
            _ => 0.,
        });

        if collisions::are_overlapping(x, y, PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT, transform.translation.x, transform.translation.y, bg_obj_width, bg_obj_height) {
            sprite.color.set_a(0.50);
        } else {
            sprite.color.set_a(1.0);
        }
    }
}

fn slide_out_of_collision(
    mut player_query: Query<&mut Player>,
    mut collisionable_query: Query<&mut CollisionComponent, Without<Player>>,
) {
    let mut player = player_query.single_mut();
    let (orig_x, orig_y) = player.get_pos();
    
    // Vérifier si le joueur est actuellement en collision avec quelque chose
    let mut currently_colliding = false;
    for collidable in collisionable_query.iter_mut() {
        if player.would_collide(orig_x, orig_y, &collidable) {
            currently_colliding = true;
            break;
        }
    }
    
    // Si le joueur n'est pas en collision, quitter la fonction
    if !currently_colliding {
        return;
    }

    // Définir les déplacements possibles
    let possible_moves = [
        (1., 0.),
        (0., 1.),
        (-1., 0.),
        (0., -1.),
        (1., 1.),
        (-1., -1.),
        (1., -1.),
        (-1., 1.)
    ];
    
    let mut best_move = (0., 0.);
    let mut min_collisions = usize::MAX;

    for (dx, dy) in possible_moves.iter() {
        let new_x = orig_x + dx;
        let new_y = orig_y + dy;
        let mut collision_count = 0;

        for collidable in collisionable_query.iter_mut() {
            if player.would_collide(new_x, new_y, &collidable) {
                collision_count += 1;
            }
        }

        if collision_count == 0 {
            best_move = (*dx, *dy);
            break;
        }

        if collision_count < min_collisions {
            best_move = (*dx, *dy);
            min_collisions = collision_count;
        }
    }

    // Déplacer le joueur vers la meilleure position trouvée
    player.set_x(orig_x + best_move.0);
    player.set_y(orig_y + best_move.1);
}


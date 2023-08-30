use std::time::Duration;

use bevy::prelude::*;

use crate::collisions;
use crate::constants::*;
use crate::collisions::*;
use crate::player::*;


pub enum EnnemyFacingDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon_ennemy)
            .add_systems(Update, (update_ennemy_position, update_ennemy_hitbox, ennemy_move_simple, ennemy_attack, despawn_on_death));  
    }
}

#[derive(Component)]
pub struct Ennemy {
    x: f32,
    y: f32,
    facingdirection: EnnemyFacingDirection,
    health: i32,
    attack: i32,
    defense_ratio: f32, // chance to block an attack
}

#[derive(Component)]
pub struct AttackDelay {
    pub timer: Timer,
}

impl AttackDelay {
    pub fn new(delay: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(delay), TimerMode::Once),
        }
    }
}

impl Ennemy {
    pub fn new(x: f32, y: f32, health: i32, attack: i32, defense_ratio: f32) -> Self {
        Self {
            x,
            y,
            health,
            facingdirection: EnnemyFacingDirection::Up,
            attack,
            defense_ratio,
        }
    }
    fn can_move(&self, direction: &PlayerFacingDirection, amount: f32, collision_query: &Query<&CollisionComponent, Without<Ennemy>>) -> bool {
        let (x, y) = match direction {
            PlayerFacingDirection::Up => (self.x, self.y + amount),
            PlayerFacingDirection::Down => (self.x, self.y - amount),
            PlayerFacingDirection::Left => (self.x - amount, self.y),
            PlayerFacingDirection::Right => (self.x + amount, self.y),
            _ => (self.x, self.y),
        };
    
        for collision in collision_query.iter() {
            if self.would_collide(x, y, collision) {
                return false;
            }
        }
    
        match direction {
            PlayerFacingDirection::Up => y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2.,
            PlayerFacingDirection::Down => y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2.,
            PlayerFacingDirection::Left => x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.,
            PlayerFacingDirection::Right => x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.,
            _ => false,
        }
    }
    
    pub fn move_in_direction(&mut self, direction: &PlayerFacingDirection, amount: f32, collision_query: &Query<&CollisionComponent, Without<Ennemy>>) -> bool {
        if self.can_move(&direction, amount, collision_query) {
            match direction {
                PlayerFacingDirection::Up => {self.y += amount; self.facingdirection = EnnemyFacingDirection::Up},
                PlayerFacingDirection::Down => {self.y -= amount; self.facingdirection = EnnemyFacingDirection::Down},
                PlayerFacingDirection::Left => {self.x -= amount; self.facingdirection = EnnemyFacingDirection::Left},
                PlayerFacingDirection::Right => {self.x += amount; self.facingdirection = EnnemyFacingDirection::Right},
                _ => (),
            }
            true
        } else {
            false
        }
    }

    fn attack(&mut self, player: &mut Player) {
        player.get_attacked(self.attack);
    }
    
    fn get_facing_direction(&self) -> &EnnemyFacingDirection {
        &self.facingdirection
    }

    pub fn get_attacked(&mut self, attack: i32) -> bool {
        if rand::random::<f32>() > self.defense_ratio {
            self.health -= attack;
            if self.health <= 0 {
                println!("ennemy died");
            }
            else {
                println!("ennemy health lowered: {}", self.health);
            }
            return true;
        }
        println!("ennemy blocked attack");
        false
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

}
    


impl Collisionable for Ennemy {
    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT)
    }
}

fn summon_ennemy(
    mut commands: Commands,
) {
    let ennemy: Ennemy = Ennemy::new(100., 0., 10, 5, 0.5);
    let hitbox = CollisionComponent::new(ennemy.x, ennemy.y, ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT);
    let attack_delay = AttackDelay::new(ENNEMY_ATTACK_DELAY);
    let entity = (SpriteBundle {
        transform: Transform {
            translation: Vec3::new(ennemy.x as f32, ennemy.y as f32, Z_LAYER_ENNEMIES),
            scale: Vec3::new(ENNEMY_SPRITE_SCALE, ENNEMY_SPRITE_SCALE, 1.),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(ENNEMY_SPRITE_SIZE, ENNEMY_SPRITE_SIZE)),
            color: Color::rgb(1., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    }, ennemy, hitbox, attack_delay);
    commands.spawn(entity);
}

fn update_ennemy_position(
    mut query: Query<(&mut Transform, &Ennemy)>,
) {
    for (mut transform, ennemy) in query.iter_mut() {
        transform.translation = Vec3::new(ennemy.x as f32, ennemy.y as f32, Z_LAYER_ENNEMIES);
    }
}

fn update_ennemy_hitbox(
    mut query: Query<(&mut CollisionComponent, &Ennemy)>,
) {
    for (mut hitbox, ennemy) in query.iter_mut() {
        hitbox.set_pos(ennemy.x, ennemy.y);
    }
}

fn ennemy_move_simple(
    mut query: Query<&mut Ennemy>,
    time: Res<Time>,
) {
    let move_amount = ENNEMY_SPEED * time.delta_seconds();

    for mut ennemy in query.iter_mut() {        
        ennemy.x += move_amount;
    }
}


fn ennemy_attack(
    mut ennemy_query: Query<(&mut Ennemy, &mut AttackDelay)>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>
) {
    let mut player = player_query.single_mut();
    for (mut ennemy, mut attack_delay) in ennemy_query.iter_mut() {

        ennemy.facingdirection = EnnemyFacingDirection::Up;

        attack_delay.timer.tick(time.delta());
        if attack_delay.timer.finished() {
            attack_delay.timer.reset();
            
            match ennemy.get_facing_direction() {
                EnnemyFacingDirection::Up => {if ennemy.would_collide(ennemy.x, ennemy.y + ENNEMY_ATTACK_RANGE, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                    }
                },
                EnnemyFacingDirection::Down => {
                    if ennemy.would_collide(ennemy.x, ennemy.y - ENNEMY_ATTACK_RANGE, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                        attack_delay.timer.tick(time.delta());
                    }
                },
                EnnemyFacingDirection::Left => {
                    if ennemy.would_collide(ennemy.x - ENNEMY_ATTACK_RANGE, ennemy.y, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                        attack_delay.timer.tick(time.delta());
                    }
                },
                EnnemyFacingDirection::Right => {
                    if ennemy.would_collide(ennemy.x + ENNEMY_ATTACK_RANGE, ennemy.y, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                        attack_delay.timer.tick(time.delta());
                    }
                },
            }
        }
    }
}

fn despawn_on_death(
    mut commands: Commands,
    mut query: Query<(Entity, &Ennemy)>,
) {
    for (entity, ennemy) in query.iter_mut() {
        if ennemy.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

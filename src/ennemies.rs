use std::time::Duration;

use bevy::prelude::*;

use crate::collisions;
use crate::constants::*;
use crate::collisions::*;
use crate::player::*;


pub enum EnnemyFacingDirection {
    Left,
    TopLeft,
    Up,
    TopRight,
    Right,
    BottomRight,
    Down,
    BottomLeft,
    
}

pub enum EnnemyState {
    Roaming,
    Chasing,
    Attacking,
    // add more states later
}

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon_ennemy)
            .add_systems(Update, (update_ennemy_position, 
                                                    update_ennemy_hitbox,
                                                    ennemy_attack, 
                                                    despawn_on_death,
                                                    ennemy_aggro_detection));  
    }
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

#[derive(Component)]
pub struct Ennemy {
    x: f32,
    y: f32,
    facingdirection: EnnemyFacingDirection,
    state: EnnemyState,
    health: i32,
    attack: i32,
    defense_ratio: f32, // chance to block an attack
}

impl Ennemy {

    pub fn new(x: f32, y: f32, health: i32, attack: i32, defense_ratio: f32) -> Self {
        Self {
            x,
            y,
            health,
            facingdirection: EnnemyFacingDirection::Up,
            state: EnnemyState::Roaming,
            attack,
            defense_ratio,
        }
    }

    fn can_move(
        &self, direction: &EnnemyFacingDirection, 
        amount: f32, 
        collision_query: &Query<&CollisionComponent, Without<Ennemy>>
    ) -> bool {
        let (x, y) = match direction {
            EnnemyFacingDirection::Up => (self.x, self.y + amount),
            EnnemyFacingDirection::Down => (self.x, self.y - amount),
            EnnemyFacingDirection::Left => (self.x - amount, self.y),
            EnnemyFacingDirection::Right => (self.x + amount, self.y),
            EnnemyFacingDirection::TopLeft => (self.x - amount, self.y + amount),
            EnnemyFacingDirection::TopRight => (self.x + amount, self.y + amount),
            EnnemyFacingDirection::BottomLeft => (self.x - amount, self.y - amount),
            EnnemyFacingDirection::BottomRight => (self.x + amount, self.y - amount),
        };
    
        for collision in collision_query.iter() {
            if self.would_collide(x, y, collision) {
                return false;
            }
        }
    
        match direction {
            EnnemyFacingDirection::Up => y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2.,
            EnnemyFacingDirection::Down => y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2.,
            EnnemyFacingDirection::Left => x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.,
            EnnemyFacingDirection::Right => x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.,
            EnnemyFacingDirection::TopLeft => {
                y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2. && x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.
            },
            EnnemyFacingDirection::TopRight => {
                y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2. && x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.
            },
            EnnemyFacingDirection::BottomLeft => {
                y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2. && x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.
            },
            EnnemyFacingDirection::BottomRight => {
                y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2. && x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.
            },
        }
    }
    
    pub fn move_in_direction(
        &mut self, direction: &EnnemyFacingDirection, 
        amount: f32, 
        collision_query: &Query<&CollisionComponent, Without<Ennemy>>
    ) -> bool {
        if self.can_move(&direction, amount, collision_query) {
            match direction {
                EnnemyFacingDirection::Up => {self.y += amount; self.facingdirection = EnnemyFacingDirection::Up},
                EnnemyFacingDirection::Down => {self.y -= amount; self.facingdirection = EnnemyFacingDirection::Down},
                EnnemyFacingDirection::Left => {self.x -= amount; self.facingdirection = EnnemyFacingDirection::Left},
                EnnemyFacingDirection::Right => {self.x += amount; self.facingdirection = EnnemyFacingDirection::Right},
                EnnemyFacingDirection::TopLeft => {self.x -= amount; self.y += amount; self.facingdirection = EnnemyFacingDirection::TopLeft},
                EnnemyFacingDirection::TopRight => {self.x += amount; self.y += amount; self.facingdirection = EnnemyFacingDirection::TopRight},
                EnnemyFacingDirection::BottomLeft => {self.x -= amount; self.y -= amount; self.facingdirection = EnnemyFacingDirection::BottomLeft},
                EnnemyFacingDirection::BottomRight => {self.x += amount; self.y -= amount; self.facingdirection = EnnemyFacingDirection::BottomRight},
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

    fn chase_player(&mut self, player: &Player, collision_query: &Query<&CollisionComponent, Without<Ennemy>>) {
        let (x, y) = player.get_pos();
        let dx = x - self.x;  // Difference in x positions
        let dy = y - self.y;  // Difference in y positions
    
        // Normalize the direction vector (dx, dy)
        let distance = (dx*dx + dy*dy).sqrt();
        let dx = dx / distance;
        let dy = dy / distance;

    
        let mut facing_direction: Option<EnnemyFacingDirection> = None;
    

        // Calculate the new proposed positions
        let new_x = self.x + dx * ENNEMY_SPEED;

        if new_x < self.x {
            facing_direction = Some(EnnemyFacingDirection::Left);
        } else if new_x > self.x {
            facing_direction = Some(EnnemyFacingDirection::Right);
        }

        let new_y = self.y + dy * ENNEMY_SPEED;

        if new_y < self.y {
            if let Some(direction) = facing_direction {
                facing_direction = Some(match direction {
                    EnnemyFacingDirection::Left => EnnemyFacingDirection::BottomLeft,
                    EnnemyFacingDirection::Right => EnnemyFacingDirection::BottomRight,
                    _ => EnnemyFacingDirection::Down,
                });
            } else {
                facing_direction = Some(EnnemyFacingDirection::Down);
            }
        } else if new_y > self.y {
            if let Some(direction) = facing_direction {
                facing_direction = Some(match direction {
                    EnnemyFacingDirection::Left => EnnemyFacingDirection::TopLeft,
                    EnnemyFacingDirection::Right => EnnemyFacingDirection::TopRight,
                    _ => EnnemyFacingDirection::Up,
                });
            } else {
                facing_direction = Some(EnnemyFacingDirection::Up);
            }
        }
        if let Some(direction) = facing_direction {
            self.move_in_direction(&direction, ENNEMY_SPEED, collision_query);
        }
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



fn ennemy_attack(
    mut ennemy_query: Query<(&mut Ennemy, &mut AttackDelay)>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>
) {
    let mut player = player_query.single_mut();
    for (mut ennemy, mut attack_delay) in ennemy_query.iter_mut() {


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
                    }
                },
                EnnemyFacingDirection::Left => {
                    if ennemy.would_collide(ennemy.x - ENNEMY_ATTACK_RANGE, ennemy.y, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                    }
                },
                EnnemyFacingDirection::Right => {
                    if ennemy.would_collide(ennemy.x + ENNEMY_ATTACK_RANGE, ennemy.y, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                    }
                },
                EnnemyFacingDirection::TopLeft => {
                    if ennemy.would_collide(ennemy.x - ENNEMY_ATTACK_RANGE, ennemy.y + ENNEMY_ATTACK_RANGE, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                    }
                },
                EnnemyFacingDirection::TopRight => {
                    if ennemy.would_collide(ennemy.x + ENNEMY_ATTACK_RANGE, ennemy.y + ENNEMY_ATTACK_RANGE, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                    }
                },
                EnnemyFacingDirection::BottomLeft => {
                    if ennemy.would_collide(ennemy.x - ENNEMY_ATTACK_RANGE, ennemy.y - ENNEMY_ATTACK_RANGE, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
                    }
                },
                EnnemyFacingDirection::BottomRight => {
                    if ennemy.would_collide(ennemy.x + ENNEMY_ATTACK_RANGE, ennemy.y - ENNEMY_ATTACK_RANGE, &player.get_collision_component()) && collisions::equals(ennemy.get_facing_direction(), player.get_relative_position(&ennemy.get_collision_component())) {
                        ennemy.attack(&mut player);
                        println!("Player health lowered");
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

fn ennemy_aggro_detection(
    mut ennemy_query: Query<(&mut Ennemy, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
    collision_query: Query<&CollisionComponent, Without<Ennemy>>
) {
    let (player, player_transform) = player_query.single();
    for (mut ennemy, transform) in ennemy_query.iter_mut() {
        let distance = transform.translation.distance(player_transform.translation);

        if distance < ENNEMY_AGGRO_DISTANCE {
                ennemy.state = EnnemyState::Chasing;
                ennemy.chase_player(&player, &collision_query);
                println!("ennemy is chasing")
            
        } else {
            ennemy.state = EnnemyState::Roaming;
            println!("ennemy is roaming")
        }
    }
}

use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::collisions;
use crate::constants::*;
use crate::collisions::*;
use crate::entitypattern::EntityBehavior;
use crate::entitypattern::EntityPatern;
use crate::entitypattern::FacingDirection;
use crate::player::*;

pub enum EnnemyState {
    Roaming,
    Chasing,
    // add more states later
}

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon_ennemies)
            .add_systems(Update, (update_ennemy_position, 
                                                    update_ennemy_hitbox,
                                                    ennemy_attack, 
                                                    despawn_on_death,
                                                    ennemy_aggro_detection,
                                                    state_speed_update));  
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

#[derive(Component)]
pub struct Ennemy {
    self_entity: EntityPatern,
    current_speed: f32,
    direction_counter: i32,
    state: EnnemyState,
    attack: i32,
    defense_ratio: f32, // chance to block an attack
}

impl Ennemy {

    pub fn new(x: f32, y: f32, health: i32, attack: i32, defense_ratio: f32) -> Self {
        Self {
            self_entity: EntityPatern::new(x, y, ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT, health),
            current_speed: ENNEMY_NORMAL_SPEED,
            direction_counter: 0,
            state: EnnemyState::Roaming,
            attack,
            defense_ratio,
        }
    }

    fn can_move(
        &self, direction: &FacingDirection, 
        amount: f32, 
        collision_query: &Query<&CollisionComponent, Without<Ennemy>>
    ) -> bool {
        let (x, y) = match direction {
            FacingDirection::Up => (self.x(), self.y() + amount),
            FacingDirection::Down => (self.x(), self.y() - amount),
            FacingDirection::Left => (self.x() - amount, self.y()),
            FacingDirection::Right => (self.x() + amount, self.y()),
            FacingDirection::TopLeft => (self.x() - amount, self.y() + amount),
            FacingDirection::TopRight => (self.x() + amount, self.y() + amount),
            FacingDirection::BottomLeft => (self.x() - amount, self.y() - amount),
            FacingDirection::BottomRight => (self.x() + amount, self.y() - amount),
        };
    
        for collision in collision_query.iter() {
            if self.would_collide(x, y, collision) {
                return false;
            }
        }
    
        match direction {
            FacingDirection::Up => y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2.,
            FacingDirection::Down => y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2.,
            FacingDirection::Left => x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.,
            FacingDirection::Right => x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.,
            FacingDirection::TopLeft => {
                y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2. && x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.
            },
            FacingDirection::TopRight => {
                y < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_HEIGHT / 2. && x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.
            },
            FacingDirection::BottomLeft => {
                y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2. && x > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_WIDTH / 2.
            },
            FacingDirection::BottomRight => {
                y > -MAP_SIZE as f32 / 2. + ENNEMY_HITBOX_HEIGHT / 2. && x < MAP_SIZE as f32 / 2. - ENNEMY_HITBOX_WIDTH / 2.
            },
        }
    }
    
    pub fn move_in_direction(
        &mut self, direction: &FacingDirection, 
        amount: f32, 
        collision_query: &Query<&CollisionComponent, Without<Ennemy>>
    ) -> bool {
        if self.can_move(&direction, amount, collision_query) {
            match direction {
                FacingDirection::Up => {self.add_y(amount); self.set_facing_direction(FacingDirection::Up)},
                FacingDirection::Down => {self.add_y(-amount); self.set_facing_direction(FacingDirection::Down)},
                FacingDirection::Left => {self.add_x(-amount); self.set_facing_direction(FacingDirection::Left)},
                FacingDirection::Right => {self.add_x(amount); self.set_facing_direction(FacingDirection::Right)},
                FacingDirection::TopLeft => {self.add_x(-amount); self.add_y(amount); self.set_facing_direction(FacingDirection::TopLeft)},
                FacingDirection::TopRight => {self.add_x(amount); self.add_y(amount); self.set_facing_direction(FacingDirection::TopRight)},
                FacingDirection::BottomLeft => {self.add_x(-amount); self.add_y(-amount); self.set_facing_direction(FacingDirection::BottomLeft)},
                FacingDirection::BottomRight => {self.add_x(amount); self.add_y(-amount); self.set_facing_direction(FacingDirection::BottomRight)},
            }
            true
        } else {
            false
        }
    }


    fn chase_player(&mut self, player: &Player, collision_query: &Query<&CollisionComponent, Without<Ennemy>>) {
        let (x, y) = player.get_pos();
        let dx = x - self.x();  // Difference in x positions
        let dy = y - self.y();  // Difference in y positions
    
        // Normalize the direction vector (dx, dy)
        let distance = (dx*dx + dy*dy).sqrt();
        let dx = dx / distance;
        let dy = dy / distance;

    
        let mut facing_direction: Option<FacingDirection> = None;
    
        let new_x = self.x() + dx * self.current_speed;

        if new_x < self.x() {
            facing_direction = Some(FacingDirection::Left);
        } else if new_x > self.x() {
            facing_direction = Some(FacingDirection::Right);
        }

        let new_y = self.y() + dy * self.current_speed;

        if new_y < self.y() {
            if let Some(direction) = facing_direction {
                facing_direction = Some(match direction {
                    FacingDirection::Left => FacingDirection::BottomLeft,
                    FacingDirection::Right => FacingDirection::BottomRight,
                    _ => FacingDirection::Down,
                });
            } else {
                facing_direction = Some(FacingDirection::Down);
            }
        } else if new_y > self.y() {
            if let Some(direction) = facing_direction {
                facing_direction = Some(match direction {
                    FacingDirection::Left => FacingDirection::TopLeft,
                    FacingDirection::Right => FacingDirection::TopRight,
                    _ => FacingDirection::Up,
                });
            } else {
                facing_direction = Some(FacingDirection::Up);
            }
        }
        if let Some(direction) = facing_direction {
            self.move_in_direction(&direction, self.current_speed, collision_query);
        }
    }
    

    fn roaming(&mut self, collision_query: &Query<&CollisionComponent, Without<Ennemy>>) {
        let new_direction: Option<FacingDirection>;
        if self.direction_counter <= 0 {
            // Choisir une nouvelle direction
            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..8);
            new_direction = Some(match direction {
                0 => FacingDirection::Up,
                1 => FacingDirection::Down,
                2 => FacingDirection::Left,
                3 => FacingDirection::Right,
                4 => FacingDirection::TopLeft,
                5 => FacingDirection::TopRight,
                6 => FacingDirection::BottomLeft,
                7 => FacingDirection::BottomRight,
                _ => FacingDirection::Up, // ou un autre par défaut
            });
            self.direction_counter = rng.gen_range(25..50); // changer de direction après 50 à 100 itérations
        }
        else {
            new_direction = self.facing_direction().clone();
        }
        if let Some(ref direction) = new_direction{
            self.move_in_direction(direction, self.current_speed, collision_query);
        }

        self.direction_counter -= 1;
    }
}


impl Collisionable for Ennemy {
    fn get_pos(&self) -> (f32, f32) {
        (self.x(), self.y())
    }

    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x(), self.y(), ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT)
    }
}

impl EntityBehavior for Ennemy {
    fn attack(&mut self, target: &mut dyn EntityBehavior) -> bool {
        return target.get_attacked(self.attack);
    }

    fn get_attacked(&mut self, damage: i32) -> bool {
        if rand::random::<f32>() > self.defense_ratio {
            self.take_damage(damage);
            if self.self_entity.health() <= 0 {
                println!("ennemy died");
            }
            else {
                println!("ennemy health lowered: {}", self.self_entity.health());
            }
            return true;
        }
        println!("ennemy blocked attack");
        false
    }

    fn take_damage(&mut self, damage: i32) -> bool {
        self.self_entity.add_health(-damage);
        true
    }

    fn x(&self) -> f32 {
        self.self_entity.x()
    }

    fn y(&self) -> f32 {
        self.self_entity.y()
    }

    fn set_x(&mut self, x: f32) {
        self.self_entity.set_x(x)
    }

    fn set_y(&mut self, y: f32) {
        self.self_entity.set_y(y)
    }

    fn add_x(&mut self, x: f32) {
        self.self_entity.add_x(x)
    }

    fn add_y(&mut self, y: f32) {
        self.self_entity.add_y(y)
    }

    fn facing_direction(&self) -> Option<FacingDirection> {
        self.self_entity.facing_direction()
    }

    fn set_facing_direction(&mut self, facing_direction: FacingDirection) {
        self.self_entity.set_facing_direction(facing_direction)
    }
}

fn summon_ennemy(
    commands: &mut Commands,
) {
    
    let mut rng = rand::thread_rng();

    let max_value_x = MAP_SIZE / 2. - SANCTUARY_WIDTH / 2.;
    let max_value_y = MAP_SIZE / 2. - SANCTUARY_HEIGHT / 2.;

    let mut x: f32;
    let mut y: f32;

    loop {
        x = rng.gen_range(-max_value_x..max_value_x);
        y = rng.gen_range(-max_value_y..max_value_y);
        if x!= 0. && y != 0. {
            break;
        }
    }

    let ennemy: Ennemy = Ennemy::new(x, y, 10, 5, 0.5);
    let hitbox = CollisionComponent::new(ennemy.x(), ennemy.y(), ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT);
    let attack_delay = AttackDelay::new(ENNEMY_ATTACK_DELAY);
    let entity = (SpriteBundle {
        transform: Transform {
            translation: Vec3::new(ennemy.x() as f32, ennemy.y() as f32, Z_LAYER_ENNEMIES),
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

fn summon_ennemies(
    mut commands: Commands,
) {
    for _ in 0..ENNEMIES_NUMBER {
        summon_ennemy(&mut commands);
    }
}

fn update_ennemy_position(
    mut query: Query<(&mut Transform, &Ennemy)>,
) {
    for (mut transform, ennemy) in query.iter_mut() {
        transform.translation = Vec3::new(ennemy.x() as f32, ennemy.y() as f32, Z_LAYER_ENNEMIES);
    }
}

fn update_ennemy_hitbox(
    mut query: Query<(&mut CollisionComponent, &Ennemy)>,
) {
    update_collisionable_pos::<Ennemy>(&mut query);
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
            if let Some(direction) = ennemy.facing_direction() {
                let actual_player: &mut Player = &mut player;
                match direction {
                    FacingDirection::Up => {
                        if ennemy.would_collide(ennemy.x(), ennemy.y() + ENNEMY_ATTACK_RANGE, &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::Down => {
                        if ennemy.would_collide(ennemy.x(), ennemy.y() - ENNEMY_ATTACK_RANGE, &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::Left => {
                        if ennemy.would_collide(ennemy.x() - ENNEMY_ATTACK_RANGE, ennemy.y(), &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::Right => {
                        if ennemy.would_collide(ennemy.x() + ENNEMY_ATTACK_RANGE, ennemy.y(), &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::TopLeft => {
                        if ennemy.would_collide(ennemy.x() - ENNEMY_ATTACK_RANGE, ennemy.y() + ENNEMY_ATTACK_RANGE, &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::TopRight => {
                        if ennemy.would_collide(ennemy.x() + ENNEMY_ATTACK_RANGE, ennemy.y() + ENNEMY_ATTACK_RANGE, &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::BottomLeft => {
                        if ennemy.would_collide(ennemy.x() - ENNEMY_ATTACK_RANGE, ennemy.y() - ENNEMY_ATTACK_RANGE, &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                    FacingDirection::BottomRight => {
                        if ennemy.would_collide(ennemy.x() + ENNEMY_ATTACK_RANGE, ennemy.y() - ENNEMY_ATTACK_RANGE, &actual_player.get_collision_component()) && collisions::equals(&direction, actual_player.get_relative_position(&ennemy.get_collision_component())) {
                            ennemy.attack(actual_player);
                        }
                    },
                }
            }
        }
    }
}

fn despawn_on_death(
    mut commands: Commands,
    mut query: Query<(Entity, &Ennemy)>,
) {
    for (entity, ennemy) in query.iter_mut() {
        if ennemy.self_entity.health() <= 0 {
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

        if distance < ENNEMY_AGGRO_DISTANCE && player.is_aggroable() {
                ennemy.state = EnnemyState::Chasing;
                ennemy.chase_player(&player, &collision_query);
            
        } else {
            ennemy.state = EnnemyState::Roaming;
            ennemy.roaming(&collision_query);
        }
    }
}

fn state_speed_update(
    mut ennemy_query: Query<&mut Ennemy>
)
 {
    for mut ennemy in ennemy_query.iter_mut() {
        match ennemy.state {
            EnnemyState::Roaming => ennemy.current_speed = ENNEMY_NORMAL_SPEED,
            EnnemyState::Chasing => ennemy.current_speed = ENNEMY_SPRINT_SPEED,
        }
    }
 }

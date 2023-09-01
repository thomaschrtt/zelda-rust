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

#[derive(Clone, Copy, PartialEq)]
pub enum EnnemyState {
    Loading,
    Idle,
    Roaming,
    Chasing,
    Damaged,
    Attacking,
    Blocking,
    Dying,
    Dead,
    // add more states later
}

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon_ennemies)
            .add_systems(Update, (game_ready.run_if(run_once()),
                                                    update_ennemy_position, 
                                                    update_ennemy_hitbox,
                                                    ennemy_attack, 
                                                    despawn_on_death,
                                                    ennemy_aggro_detection,
                                                    state_speed_update,
                                                    update_ennemy_sprite,
                                                    change_sprite_orientation));  
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

    roaming_frame_counter: usize,
    roaming_frame_time: f32,

    chasing_frame_counter: usize,
    chasing_frame_time: f32,

    damaged_frame_counter: usize,
    damaged_frame_time: f32,

    attacking_frame_counter: usize,
    attacking_frame_time: f32,
    attacking_has_hit: bool,

    blocking_frame_counter: usize,
    blocking_frame_time: f32,

    dying_frame_counter: usize,
    dying_frame_time: f32,

}

impl Ennemy {

    pub fn new(x: f32, y: f32, health: i32, attack: i32, defense_ratio: f32) -> Self {
        Self {
            self_entity: EntityPatern::new(x, y, ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT, health),
            current_speed: ENNEMY_NORMAL_SPEED,
            direction_counter: 0,
            state: EnnemyState::Loading,
            attack,
            defense_ratio,

            roaming_frame_counter: 0,
            roaming_frame_time: 0.,

            chasing_frame_counter: 0,
            chasing_frame_time: 0.,

            damaged_frame_counter: 0,
            damaged_frame_time: 0.,

            attacking_frame_counter: 0,
            attacking_frame_time: 0.,
            attacking_has_hit: false,

            blocking_frame_counter: 0,
            blocking_frame_time: 0.,

            dying_frame_counter: 0,
            dying_frame_time: 0.,

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
        self.state = EnnemyState::Chasing;
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
            let direction = rng.gen_range(0..17);
            new_direction = Some(match direction {
                0 => FacingDirection::Up,
                1 => FacingDirection::Down,
                2 => FacingDirection::Left,
                3 => FacingDirection::Right,
                4 => FacingDirection::TopLeft,
                5 => FacingDirection::TopRight,
                6 => FacingDirection::BottomLeft,
                7 => FacingDirection::BottomRight,
                _ => {self.state = EnnemyState::Idle; FacingDirection::Up},
            });
            if direction < 8 {
                self.state = EnnemyState::Roaming;
            }
            self.direction_counter = rng.gen_range(25..50); // changer de direction après 50 à 100 itérations
        }
        else {
            new_direction = self.facing_direction().clone();
        }
        if !self.is_idle() {
            if let Some(ref direction) = new_direction{
                self.move_in_direction(direction, self.current_speed, collision_query);
            }
        }

        self.direction_counter -= 1;
    }

    fn is_taking_damage(&self) -> bool {
        self.state == EnnemyState::Damaged
    }

    fn is_attacking(&self) -> bool {
        self.state == EnnemyState::Attacking
    }

    fn is_blocking(&self) -> bool {
        self.state == EnnemyState::Blocking
    }

    fn is_dying(&self) -> bool {
        self.state == EnnemyState::Dying
    }


    fn is_doing_something(&self) -> bool {
        self.is_taking_damage() || self.is_attacking() || self.is_blocking() || self.is_dying() || self.is_dead()
    }

    fn is_dead(&self) -> bool {
        self.state == EnnemyState::Dead
    }

    fn is_loading(&self) -> bool {
        self.state == EnnemyState::Loading
    }

    fn is_idle(&self) -> bool {
        self.state == EnnemyState::Idle
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
        if !self.is_taking_damage() {self.state = EnnemyState::Attacking;}
        if self.attacking_frame_counter == 6 && !self.attacking_has_hit{
            self.attacking_has_hit = true;
            return target.get_attacked(self.attack);
        }
        false
    }

    fn get_attacked(&mut self, damage: i32) -> bool {
        if rand::random::<f32>() > self.defense_ratio {
            self.take_damage(damage);
            if self.self_entity.health() <= 0 {
                println!("ennemy died");
                self.state = EnnemyState::Dying;
            }
            else {
                println!("ennemy health lowered: {}", self.self_entity.health());
            }
            return true;
        }
        self.state = EnnemyState::Blocking;
        println!("ennemy blocked attack");
        false
    }

    fn take_damage(&mut self, damage: i32) -> bool {
        self.self_entity.add_health(-damage);
        self.state = EnnemyState::Damaged;
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
    asset_server: &Res<AssetServer>, 
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>
) {
    
    let texture_handle = asset_server.load("Skeleton/Idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);



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

    let ennemy: Ennemy = Ennemy::new(x, y, 10, 5, 0.20);
    let hitbox = CollisionComponent::new(ennemy.x(), ennemy.y(), ENNEMY_HITBOX_WIDTH, ENNEMY_HITBOX_HEIGHT);
    let attack_delay = AttackDelay::new(ENNEMY_ATTACK_DELAY);
    let entity = (SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            translation: Vec3::new(ennemy.x() as f32, ennemy.y() as f32, Z_LAYER_ENNEMIES),
            scale: Vec3::new(ENNEMY_SPRITE_SCALE, ENNEMY_SPRITE_SCALE, 1.),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(0),
        ..Default::default()
    }, ennemy, hitbox, attack_delay);
    commands.spawn(entity);
}

fn summon_ennemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    for _ in 0..ENNEMIES_NUMBER {
        summon_ennemy(&mut commands, &asset_server, &mut texture_atlases);
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
    mut ennemy_query: Query<&mut Ennemy>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut();
    for mut ennemy in ennemy_query.iter_mut() {
        if !ennemy.is_blocking() && !ennemy.is_dying() && !ennemy.is_dead() && !ennemy.is_loading() {
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
        if ennemy.is_dead() {
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
        if !ennemy.is_doing_something() {
            if distance < ENNEMY_AGGRO_DISTANCE && player.is_aggroable() {
                ennemy.chase_player(&player, &collision_query);
            
            } else {
                ennemy.roaming(&collision_query);
            }
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
            _ => (),
        }
    }
 }

 fn update_ennemy_sprite(
    mut query: Query<(&mut Ennemy, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    for (mut ennemy, mut sprite, mut texture) in query.iter_mut() {
        match ennemy.state {
            EnnemyState::Idle => {
                let texture_handle = asset_server.load("Skeleton/Idle.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                *texture = texture_atlas_handle.clone();
                
                ennemy.roaming_frame_time += time.delta_seconds();
                if ennemy.roaming_frame_time >= 0.3 {
                    ennemy.roaming_frame_counter += 1;
                    ennemy.roaming_frame_time = 0.;
                }

                if ennemy.roaming_frame_counter >= 4 {
                    ennemy.roaming_frame_counter = 0;
                }

                sprite.index = ennemy.roaming_frame_counter;
            },
            EnnemyState::Roaming => {
                
                let texture_handle = asset_server.load("Skeleton/Walk.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                *texture = texture_atlas_handle.clone();
                
                ennemy.roaming_frame_time += time.delta_seconds();
                if ennemy.roaming_frame_time >= 0.3 {
                    ennemy.roaming_frame_counter += 1;
                    ennemy.roaming_frame_time = 0.;
                }

                if ennemy.roaming_frame_counter >= 4 {
                    ennemy.roaming_frame_counter = 0;
                }

                sprite.index = ennemy.roaming_frame_counter;
            },
            EnnemyState::Chasing => {
                    
                    let texture_handle = asset_server.load("Skeleton/Walk.png");
                    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
                    *texture = texture_atlas_handle.clone();
                    
                    ennemy.chasing_frame_time += time.delta_seconds();
                    if ennemy.chasing_frame_time >= 0.1 {
                        ennemy.chasing_frame_counter += 1;
                        ennemy.chasing_frame_time = 0.;
                    }
    
                    if ennemy.chasing_frame_counter >= 4 {
                        ennemy.chasing_frame_counter = 0;
                    }
    
                    sprite.index = ennemy.chasing_frame_counter;
            },
            EnnemyState::Damaged => {
                let texture_handle = asset_server.load("Skeleton/Take Hit.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                *texture = texture_atlas_handle.clone();
                
                ennemy.damaged_frame_time += time.delta_seconds();
                if ennemy.damaged_frame_time >= 0.2 {
                    ennemy.damaged_frame_counter += 1;
                    ennemy.damaged_frame_time = 0.;
                }

                if ennemy.damaged_frame_counter >= 4 {
                    ennemy.damaged_frame_counter = 0;
                    ennemy.state = EnnemyState::Chasing;
                }   

                sprite.index = ennemy.damaged_frame_counter;
            },
            EnnemyState::Attacking => {
                let texture_handle = asset_server.load("Skeleton/Attack.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 8, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                *texture = texture_atlas_handle.clone();
                
                ennemy.attacking_frame_time += time.delta_seconds();
                if ennemy.attacking_frame_time >= 1.4/8. {
                    ennemy.attacking_frame_counter += 1;
                    ennemy.attacking_frame_time = 0.;
                }

                if ennemy.attacking_frame_counter >= 8 {
                    ennemy.attacking_frame_counter = 0;
                    ennemy.attacking_has_hit = false;
                    ennemy.state = EnnemyState::Chasing;
                }   

                sprite.index = ennemy.attacking_frame_counter;
            },
            EnnemyState::Blocking => {
                let texture_handle = asset_server.load("Skeleton/Shield.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                *texture = texture_atlas_handle.clone();
                
                ennemy.blocking_frame_time += time.delta_seconds();
                if ennemy.blocking_frame_time >= 0.1 {
                    ennemy.blocking_frame_counter += 1;
                    ennemy.blocking_frame_time = 0.;
                }

                if ennemy.blocking_frame_counter >= 4 {
                    ennemy.blocking_frame_counter = 0;
                    ennemy.state = EnnemyState::Chasing;
                }   

                sprite.index = ennemy.blocking_frame_counter;
            },
            EnnemyState::Dying => {
                let texture_handle = asset_server.load("Skeleton/Death.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(150., 150.), 4, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                *texture = texture_atlas_handle.clone();
                
                ennemy.dying_frame_time += time.delta_seconds();
                if ennemy.dying_frame_time >= 0.2 {
                    ennemy.dying_frame_counter += 1;
                    ennemy.dying_frame_time = 0.;
                }

                if ennemy.dying_frame_counter >= 4 {
                    ennemy.dying_frame_counter = 0;
                    ennemy.state = EnnemyState::Dead;
                }   

                sprite.index = ennemy.dying_frame_counter;
            },
            EnnemyState::Dead => {
                sprite.index = 3;
            },
            EnnemyState::Loading => {
                sprite.index = 0;
            },
        }
    }
}

fn change_sprite_orientation(
    mut query: Query<(&mut Transform, &Ennemy)>,
) {
    for (mut transform, ennemy) in query.iter_mut() {
        if let Some(direction) = ennemy.facing_direction() {
            match direction {
                FacingDirection::Left => {
                    transform.scale.x = -ENNEMY_SPRITE_SCALE;
                },
                FacingDirection::Right => {
                    transform.scale.x = ENNEMY_SPRITE_SCALE;
                },
                FacingDirection::TopLeft => {
                    transform.scale.x = -ENNEMY_SPRITE_SCALE;
                },
                FacingDirection::TopRight => {
                    transform.scale.x = ENNEMY_SPRITE_SCALE;
                },
                FacingDirection::BottomLeft => {
                    transform.scale.x = -ENNEMY_SPRITE_SCALE;
                },
                FacingDirection::BottomRight => {
                    transform.scale.x = ENNEMY_SPRITE_SCALE;
                },
                _ => (),
            }
        }
    }
}

fn game_ready(
    mut ennemy_query: Query<&mut Ennemy>,
) {
    for mut ennemy in ennemy_query.iter_mut() {
        if ennemy.is_loading() {
            ennemy.state = EnnemyState::Roaming;
        }
    }
}

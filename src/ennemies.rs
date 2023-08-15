use bevy::prelude::*;

use crate::constants::*;
use crate::collisions::*;
use crate::player::*;


pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, summon_ennemy)
            .add_systems(Update, (update_ennemy_position, update_ennemy_hitbox, ennemy_move_simple));  
    }
}

#[derive(Component)]
pub struct Ennemy {
    x: f32,
    y: f32,
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
                PlayerFacingDirection::Up => self.y += amount,
                PlayerFacingDirection::Down => self.y -= amount,
                PlayerFacingDirection::Left => self.x -= amount,
                PlayerFacingDirection::Right => self.x += amount,
                _ => (),
            }
            true
        } else {
            false
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
    }, ennemy, hitbox);
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

use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;
use crate::collisions::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_structures)
            .add_systems(Update, (update_structures_pos));
    }
}

#[derive(Component, Clone)]
pub struct Sanctuary {
    x: i32,
    y: i32,
}

impl Sanctuary {
    pub fn new(x: i32, y: i32) -> Self {
        Sanctuary { x, y }
    }
    pub fn new_random_position() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-250..250);
        let y = rng.gen_range(-250..250);
        Sanctuary::new(x, y)
    }
}

impl Collisionable for Sanctuary {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, SANCTUARY_SIZE as i32, SANCTUARY_SIZE as i32)
    }
}


#[derive(Component, Clone)]
pub struct Tower {
    x: i32,
    y: i32,
    sanctuaries: Vec<Sanctuary>,
}


impl Tower {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut sanctuaries: Vec<Sanctuary> = Vec::new();
        unsafe { sanctuaries.set_len(4) };
        Tower { x, y, sanctuaries}
    }

}

impl Collisionable for Tower {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, TOWER_SIZE as i32, TOWER_SIZE as i32)
    }
}   

pub fn setup_structures(mut commands: Commands, collision_query: Query<&CollisionComponent>) {
    let tower = Tower::new(100, 100);
    
    // Setup sanctuaries
    for _ in 0..tower.sanctuaries.len() {
        let mut attempts = 0;
        let max_attempts = 10; // Or whatever number you deem reasonable.


        let mut added_sanctuaries: Vec<CollisionComponent> = Vec::new();
        
        loop {
            attempts += 1;
            
            let sanctuary = Sanctuary::new_random_position();
            let collisioncomponent = CollisionComponent::new_from_component(&sanctuary);

            // Check for collisions with existing entities.
            let mut collides = false;
            for existing in collision_query.iter() {
                if sanctuary.would_collide_with(existing) {
                    collides = true;
                    break;
                }
            }

            // Check for collisions with sanctuaries we've added this frame.
            if !collides {
                for added_sanctuary in added_sanctuaries.iter() {
                    if sanctuary.would_collide_with(added_sanctuary) {
                        collides = true;
                        break;
                    }
                }
            }

            if !collides || attempts >= max_attempts {
                if !collides {
                    added_sanctuaries.push(collisioncomponent.clone());
                    commands.spawn((SpriteBundle {
                        transform: Transform::from_xyz(sanctuary.x as f32, sanctuary.y as f32, 1.0),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(SANCTUARY_SIZE, SANCTUARY_SIZE)),
                            color: Color::rgb(0.0, 1.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }, collisioncomponent, sanctuary));
                }
                break;
            }
        }
    }

    // Setup tower
    let collisioncomponent = CollisionComponent::new_from_component(&tower);
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(TOWER_SIZE, TOWER_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    }, tower, collisioncomponent));
}


fn update_structures_pos(mut query: Query<(&mut Transform, &Tower)>) {
    for (mut transform, tower) in query.iter_mut() {
        transform.translation.x = tower.x as f32;
        transform.translation.y = tower.y as f32;
    }
}



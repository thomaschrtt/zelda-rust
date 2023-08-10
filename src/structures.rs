use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;
use crate::collisions::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_structures)
            .add_systems(Update, (update_structures_pos, remove_sanctuaries_colliding_with_tower));
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
        (self.x, self.y, SANCTUARY_SIZE as i32 + 5, SANCTUARY_SIZE as i32 + 5)
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
        let mut sanctuaries: Vec<Sanctuary> = Vec::new();
        unsafe { sanctuaries.set_len(5) };
        Tower { x, y, sanctuaries}
    }

}

impl Collisionable for Tower {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, TOWER_WIDTH as i32, TOWER_HEIGHT as i32)
    }
}   
fn does_collide_with_existing(sanctuary: &Sanctuary, query: &Query<&CollisionComponent>, added_sanctuaries: &[CollisionComponent]) -> bool {
    query.iter().any(|existing| sanctuary.would_collide_with(existing))
        || added_sanctuaries.iter().any(|added| sanctuary.would_collide_with(added))
}

fn setup_sanctuary(commands: &mut Commands, tower: &Tower, collision_query: &Query<&CollisionComponent>) {
    let mut added_sanctuaries = Vec::new();

    for _ in 0..tower.sanctuaries.len() {
        let sanctuary = (0..10)
            .map(|_| Sanctuary::new_random_position())
            .find(|sanct| !does_collide_with_existing(sanct, &collision_query, &added_sanctuaries))
            .unwrap_or_else(Sanctuary::new_random_position);  // Use a default position if all attempts failed.

        let collision_component = CollisionComponent::new_from_component(&sanctuary);
        added_sanctuaries.push(collision_component.clone());

        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(sanctuary.x as f32, sanctuary.y as f32, 2.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SANCTUARY_SIZE, SANCTUARY_SIZE)),
                color: Color::rgb(0.0, 1.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(collision_component)
        .insert(sanctuary);
    }
}

pub fn setup_structures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_query: Query<&CollisionComponent>
) {
    let tower_texture_handle = asset_server.load("tower.png");

    let tower = Tower::new(100, 100);
    setup_sanctuary(&mut commands, &tower, &collision_query);

    // Setup tower
    let collisioncomponent = CollisionComponent::new_from_component(&tower);
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(TOWER_WIDTH, TOWER_HEIGHT)),
            ..Default::default()
        },
        texture: tower_texture_handle,
        ..Default::default()
    })
    .insert(tower)
    .insert(collisioncomponent);
}


fn remove_sanctuaries_colliding_with_tower(
    mut commands: Commands, 
    tower: Query<&Tower>,
    mut sanctuary_query: Query<(Entity, &Sanctuary, &CollisionComponent)>) 
    {
        for towers in tower.iter() {
            for (entity, _, collision_component) in sanctuary_query.iter_mut() {
                if towers.would_collide_with(collision_component) {
                    commands.entity(entity).despawn();
                }
            }
        }
}

fn update_structures_pos(mut query: Query<(&mut Transform, &Tower)>) {
    for (mut transform, tower) in query.iter_mut() {
        transform.translation = Vec3::new(tower.x as f32, tower.y as f32, transform.translation.z);
    }
}



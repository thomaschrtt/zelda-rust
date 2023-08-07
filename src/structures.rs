use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;
use crate::collisions::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_structures)
            .add_systems(Update, (update_structures_pos, show_hitbox));
    }
}

#[derive(Component)]
pub struct Sanctuary {
    x: i32,
    y: i32,
}

impl Sanctuary {
    pub fn new(x: i32, y: i32) -> Self {
        Sanctuary { x, y }
    }
}


#[derive(Component)]
pub struct Tower {
    x: i32,
    y: i32,
    sanctuaries: Vec<Sanctuary>,
}


impl Tower {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut sanctuaries: Vec<Sanctuary> = Vec::new();
        for _ in 0..5 {
            let x_sanct_pos = rng.gen_range(-250..250);
            let y_sanct_pos = rng.gen_range(-250..250);
            sanctuaries.push(Sanctuary::new(x_sanct_pos, y_sanct_pos));
        }
        Tower { x, y, sanctuaries}
    }

}

impl Collisionable for Tower {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, STRUCT_SIZE as i32, STRUCT_SIZE as i32)
    }
}   

pub fn setup_structures(mut commands: Commands) {
    let tower = Tower::new(100,100);
    let collisioncomponent = CollisionComponent::new_from_component(&tower);
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(STRUCT_SIZE, STRUCT_SIZE)),
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

fn show_hitbox(
    mut commands: Commands,
    mut query: Query<(&Tower, &Transform)>,
    mut keyInput: ResMut<Input<KeyCode>>,
) {
    if keyInput.just_pressed(KeyCode::H) {
        let tower = query.single_mut();
        let (x, y, w, h) = tower.0.get_hitbox();
        println!("x: {}, y: {}, w: {}, h: {}", x, y, w, h);
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(x as f32, y as f32, 4.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(w as f32, h as f32)),
                color: Color::rgba(1., 0., 0., 0.5),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

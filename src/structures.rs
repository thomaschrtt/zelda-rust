use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_structures)
            .add_systems(Update, update_structures_pos);
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
        for i in 0..5 {
            let x_sanct_pos = rng.gen_range(-250..250);
            let y_sanct_pos = rng.gen_range(-250..250);
            sanctuaries.push(Sanctuary::new(x_sanct_pos, y_sanct_pos));
        }

        Tower { x, y, sanctuaries}
    }

    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-250..250);
        let y = rng.gen_range(-250..250);
        Tower::new(x, y)
    }
}

pub fn setup_structures(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(STRUCT_SIZE, STRUCT_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    }, Tower::new_random()));
}

fn update_structures_pos(mut query: Query<(&mut Transform, &Tower)>) {
    for (mut transform, tower) in query.iter_mut() {
        transform.translation.x = tower.x as f32;
        transform.translation.y = tower.y as f32;
    }
}

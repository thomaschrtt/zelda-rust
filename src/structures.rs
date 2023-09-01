use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;
use crate::collisions::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_structures)
            .add_systems(Update, (update_structures_pos, 
                                                    remove_sanctuaries_colliding_with_tower, 
                                                    update_visibility, 
                                                    change_visibility_with_keybinding, 
                                                    update_collision_component, 
                                                    hide_all_sanctuaries.run_if(run_once()),
                                                    update_sanctuary_color));
    }
}

#[derive(Component, Clone)]
pub struct Tower {
    x: f32,
    y: f32,
}


impl Tower {
    pub fn new(x: f32, y: f32) -> Self {
        Tower { x, y}
    }

}

impl Collisionable for Tower {
    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, TOWER_WIDTH, TOWER_HEIGHT)
    }
}  

#[derive(Component, Clone)]
pub struct Sanctuary {
    x: f32,
    y: f32,
    visibility: bool,
    unlocked: bool,
}

impl Sanctuary {
    pub fn new(x: f32, y: f32) -> Self {
        Sanctuary { x, y, visibility: true, unlocked: false }
    }
    pub fn new_random_position() -> Self {
        let mut rng = rand::thread_rng();

        let max_value_x = MAP_SIZE / 2. - SANCTUARY_WIDTH / 2.;
        let max_value_y = MAP_SIZE / 2. - SANCTUARY_HEIGHT / 2.;

        let x = rng.gen_range(-max_value_x..max_value_x);
        let y = rng.gen_range(-max_value_y..max_value_y);
        Sanctuary::new(x, y)
    }

    pub fn unlock(&mut self) -> bool {
        if self.unlocked {
            println!("Sanctuaire déjà débloqué");
            return false;
        }
        println!("Sanctuaire débloqué");
        self.unlocked = true;
        true
    }

    pub fn is_visible(&self) -> bool {
        self.visibility
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }
}

impl Collisionable for Sanctuary {
    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        if self.visibility {
            (self.x, self.y, SANCTUARY_WIDTH, SANCTUARY_HEIGHT)
        } else {
            (-1000., -1000., 0., 0.)
        }
    }
}

fn does_collide_with_existing(sanctuary: &Sanctuary, query: &Query<&CollisionComponent>, added_sanctuaries: &[CollisionComponent]) -> bool {
    query.iter().any(|existing| sanctuary.would_collide_with(existing))
        || added_sanctuaries.iter().any(|added| sanctuary.would_collide_with(added))
}

fn setup_sanctuary(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    collision_query: &Query<&CollisionComponent>
) {
    // Load the sanctuary texture
    let texture_handle = asset_server.load("sanctuary.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SANCTUARY_WIDTH, SANCTUARY_HEIGHT), 2, 1, Some(Vec2 { x: 1., y: 0. }), Some(Vec2::new(0., 0.))); // Assuming two textures side by side.
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut added_sanctuaries = Vec::new();

    for _ in 0..SANCTUARY_NB {
        if let Some(sanctuary) = (0..10)
            .map(|_| Sanctuary::new_random_position())
            .find(|sanct| !does_collide_with_existing(sanct, &collision_query, &added_sanctuaries)) {

            let collision_component = CollisionComponent::new_from_component(&sanctuary);
            added_sanctuaries.push(collision_component.clone());

            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(sanctuary.x as f32, sanctuary.y as f32, Z_LAYER_STRUCTURES),
                sprite: TextureAtlasSprite { index: 0, ..Default::default() }, // Use the first texture (red one)
                ..Default::default()
            })
            .insert(collision_component)
            .insert(sanctuary);
        }
    }
}


pub fn setup_structures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collision_query: Query<&CollisionComponent>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tower_texture_handle = asset_server.load("tower.png");

    let tower = Tower::new(100., 100.);
    setup_sanctuary(&mut commands, asset_server, texture_atlases, &collision_query);

    // Setup tower
    let collisioncomponent = CollisionComponent::new_from_component(&tower);
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, Z_LAYER_STRUCTURES),
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

fn update_visibility(mut query: Query<(&mut Visibility, &Sanctuary)>) {
    for (mut sprite_visibility, sanctuary) in query.iter_mut() {
        *sprite_visibility = if sanctuary.visibility { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn change_visibility_with_keybinding(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Sanctuary>,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        for mut sanctuary in query.iter_mut() {
            sanctuary.visibility = !sanctuary.visibility;
        }
    }
}

fn update_collision_component(mut query: Query<(&mut CollisionComponent, &Sanctuary)>) {
    for (mut collision_component, sanctuary) in query.iter_mut() {
        collision_component.update_hitbox(sanctuary);
    }
}

fn hide_all_sanctuaries(mut query: Query<&mut Sanctuary>) {
    for mut sanctuary in query.iter_mut() {
        sanctuary.visibility = false;
    }
}
pub fn show_one_sanctuary(mut query: Query<&mut Sanctuary>) {
    if are_all_visible_sanctuaries_unlocked(&query) {

        let mut rng = rand::thread_rng();
        let mut sanctuaries: Vec<_> = query.iter_mut().filter(|sanctuary| !sanctuary.visibility).collect();
        let len = sanctuaries.len();

        if len > 0 {
            let sanctuary = &mut sanctuaries[rng.gen_range(0..len)];
            sanctuary.visibility = true;
        } else {
            println!("Pas de sanctuaire trouvé");
        }
    }
    else {
        println!("Tous les sanctuaires ne sont pas débloqués");
    }

}
pub fn are_all_visible_sanctuaries_unlocked(query: &Query<&mut Sanctuary>) -> bool {
    query.iter().filter(|sanctuary| sanctuary.visibility).all(|sanctuary| sanctuary.unlocked)
}

fn update_sanctuary_color(mut query: Query<(&mut TextureAtlasSprite, &Sanctuary)>) {
    for (mut sprite, sanctuary) in query.iter_mut() {
        sprite.index = if sanctuary.unlocked { 1 } else { 0 };
    }
}


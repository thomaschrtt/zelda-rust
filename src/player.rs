use bevy::prelude::*;
use crate::constants::*;
use crate::collisions::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, (player_move, player_facing_direction, update_player_pos, show_hitbox));
    }
}


#[derive(Component)]
pub struct Player {
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Collisionable for Player {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, (PLAYER_HITBOX_WIDTH * 0.8) as i32, (PLAYER_HITBOX_HEIGHT*0.8) as i32)
    }
}





fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    collisionable_query: Query<&CollisionComponent>,
) {

    let mut player = player_query.single_mut();

    let left_boundary = -((WINDOW_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.)) as i32;
    let right_boundary = -left_boundary;
    let top_boundary = right_boundary;
    let bottom_boundary = left_boundary;

    // Calculez les nouvelles positions potentielles.
    let new_x = if keyboard_input.pressed(KeyCode::Left) { player.x - PLAYER_SPEED }
                     else if keyboard_input.pressed(KeyCode::Right) { player.x + PLAYER_SPEED }
                     else { player.x };

    let new_y = if keyboard_input.pressed(KeyCode::Down) { player.y - PLAYER_SPEED }
                     else if keyboard_input.pressed(KeyCode::Up) { player.y + PLAYER_SPEED }
                     else { player.y };


    for collidable in collisionable_query.iter() {
        if player.would_collide(new_x, new_y, collidable){ 
            return;  
        }
    }
    if new_x < left_boundary || new_x > right_boundary || new_y < bottom_boundary || new_y > top_boundary {
        return;
    }

    // Si aucune collision n'est détectée, appliquez le mouvement.
    player.x = new_x;
    player.y = new_y;
}
    

fn player_facing_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
        player.scale.x = -PLAYER_SPRITE_SCALE;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player.scale.x = PLAYER_SPRITE_SCALE;
    }
}


fn update_player_pos(
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let mut player = query.single_mut();
    let x = player.0.x as f32;
    let y = player.0.y as f32;
    player.1.translation.x = x;
    player.1.translation.y = y;
}

fn spawn_player(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) 
    {

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE), 4, 4, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player: Player = Player::new();

    commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, 1.),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(player);
}
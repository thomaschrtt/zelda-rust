use bevy::prelude::*;

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 4.;
const PLAYER_SPRITE_SIZE: f32 = 32.;
const PLAYER_SPRITE_SCALE: f32 = 1.5;
const PLAYER_HITBOX_WIDTH: f32 = PLAYER_SPRITE_SIZE * PLAYER_SPRITE_SCALE; // rendered hitbox 
const PLAYER_HITBOX_HEIGHT: f32 = PLAYER_SPRITE_SIZE * PLAYER_SPRITE_SCALE; // rendered hitbox


const WINDOW_SIZE: f32 = 600.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_move, player_facing_direction, update_player_pos));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player{
    x : i32,
    y : i32,
}

#[derive(Component)]
struct Tower {
    x : i32,
    y : i32,
}

#[derive(Component)]
struct Sanctuary {
    x : i32,
    y : i32,
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();
    window.resolution.set(WINDOW_SIZE, WINDOW_SIZE);
    window.resize_constraints = WindowResizeConstraints {
        min_width: WINDOW_SIZE,
        max_width: WINDOW_SIZE,
        min_height: WINDOW_SIZE,
        max_height: WINDOW_SIZE,
    };
    window.resizable = false;

    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Transform::default()
        },
        ..Default::default()
    });

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE), 4, 4, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
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
        .insert(Player { x: 0, y: 0 });
}

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let left_boundary = (-((WINDOW_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.)) / PLAYER_SPEED) as i32;
    let right_boundary = -left_boundary;
    let top_boundary = right_boundary;
    let bottom_boundary = left_boundary;

    let mut player = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) && player.x > left_boundary {
        player.x -= 1;
    }
    if keyboard_input.pressed(KeyCode::Right) && player.x < right_boundary {
        player.x += 1;
    }
    if keyboard_input.pressed(KeyCode::Up) && player.y < top_boundary {
        player.y += 1;
    }
    if keyboard_input.pressed(KeyCode::Down) && player.y > bottom_boundary {
        player.y -= 1;
    }
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
    player.1.translation.x = player.0.x as f32 * PLAYER_SPEED;
    player.1.translation.y = player.0.y as f32 * PLAYER_SPEED;
}
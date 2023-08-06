use bevy::prelude::*;
use crate::constants::*;
use crate::player::Player;


pub fn setup(
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
        .insert(Player::new());
}
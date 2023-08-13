use bevy::prelude::*;
use rand::Rng;
use crate::{constants::*, player::*};


pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
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

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale : CAMERA_DEFAULT_SCALE,
            ..OrthographicProjection::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., Z_LAYER_BACKGROUND),
            ..Transform::default()
        },
        ..Default::default()
    });

}

pub fn zoom_camera(
    mut query: Query<&mut OrthographicProjection>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::S) && transform.scale < CAMERA_MAX_SCALE {
        transform.scale = transform.scale + 0.01;
    }
    if keyboard_input.pressed(KeyCode::Z) && transform.scale > CAMERA_MIN_SCALE{
        transform.scale = transform.scale - 0.01;
    }
    if keyboard_input.pressed(KeyCode::R) {
        transform.scale = CAMERA_DEFAULT_SCALE;
    }
    if keyboard_input.pressed(KeyCode::M) {
        transform.scale = CAMERA_MAX_SCALE;
    }

}

pub fn track_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Camera, &mut Transform), Without<Player>>,
    mut camera_proj : Query<&mut OrthographicProjection, With<Camera>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera.single_mut().1;
    let camera_projection = camera_proj.single_mut();

    let camera_range = camera_projection.scale * WINDOW_SIZE / 2.;
    
    let x = player_transform.translation.x;
    let y = player_transform.translation.y;

    let camera_max_x = MAP_SIZE / 2. - camera_range;
    let camera_min_x = -MAP_SIZE / 2. + camera_range;
    let camera_max_y = MAP_SIZE / 2. - camera_range;
    let camera_min_y = -MAP_SIZE / 2. + camera_range;

    camera_transform.translation.x = if x > camera_max_x { camera_max_x } else if x < camera_min_x { camera_min_x } else { x };
    camera_transform.translation.y = if y > camera_max_y { camera_max_y } else if y < camera_min_y { camera_min_y } else { y };
}

pub fn setup_random_trees(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tree_texture_handle = asset_server.load("trees.png");
    let tree_texture_atlas = TextureAtlas::from_grid(tree_texture_handle, Vec2::new(TREE_WIDTH, TREE_HEIGHT), 3, 1, Some(Vec2::new(20., 0.)), Some(Vec2::new(0., 0.)));
    let tree_texture_atlas_handle = texture_atlases.add(tree_texture_atlas);

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        let y = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        let index = rng.gen_range(0..3);
        println!("x: {}, y: {}, index: {}", x, y, index);
        commands.spawn(SpriteSheetBundle {
            texture_atlas: tree_texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, -y+MAP_SIZE/2.),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        });
    }
}
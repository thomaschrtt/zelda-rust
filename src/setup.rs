use bevy::prelude::*;
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
            scale : 1.,
            ..OrthographicProjection::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
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
    if keyboard_input.pressed(KeyCode::S) && transform.scale < 1. {
        transform.scale = transform.scale + 0.01;
    }
    if keyboard_input.pressed(KeyCode::Z) && transform.scale > 0.1{
        transform.scale = transform.scale - 0.01;
    }
}

pub fn track_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Camera, &mut Transform), Without<Player>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera.single_mut().1;
    camera_transform.translation = Vec3::new(player_transform.translation.x, player_transform.translation.y, camera_transform.translation.z);
}
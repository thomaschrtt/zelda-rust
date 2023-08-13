use bevy::prelude::*;
use crate::{constants::*, structures::Sanctuary, collisions::{*, self}};

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_gui)
            .add_systems(Update, (update_visibility, 
                                                    update_gui_pos,
                                                    update_display_pos));
    }
}

#[derive(Component, Clone)]
pub struct GUI {
    x: f32,
    y: f32,
    color: Color,
    visible: bool,
}

impl GUI {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        GUI { x, y, color, visible: false }
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

fn setup_gui(mut commands: Commands, 
    asset_server: Res<AssetServer>,) {
    let gui = GUI::new(0., 0., Color::rgb(0.0, 0.0, 1.0));
    let texture = asset_server.load("s.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: gui.color,
            custom_size: Some(Vec2::new(10., 10.)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., Z_LAYER_GUI),
            ..Transform::default()
        },
        texture: texture,
        ..Default::default()
    }, gui));
}

fn update_gui_pos(mut query: Query<&mut GUI>,  
                  visible_sanctuary_query: Query<&Sanctuary>, 
                  camera_pos: Query<&Transform, With<Camera>>) 
{
    let mut gui = query.single_mut();

    let camera_pos = camera_pos.single();
    let camera_pos = (camera_pos.translation.x as i32, camera_pos.translation.y as i32);

    let sanct_pos: Vec<&Sanctuary> = visible_sanctuary_query.iter()
    .filter(|sanctuary| sanctuary.is_visible() && !sanctuary.is_unlocked())
    .collect();
    let sanct_pos = if sanct_pos.len() > 0 {sanct_pos[0].get_pos()} else {gui.visible = false; return};


    if is_sanct_visible(sanct_pos.0, sanct_pos.1, camera_pos.0, camera_pos.1) {
        gui.set_visible(false);
    } else {
        gui.set_visible(true);
    }

    let (x, y) = get_gui_pos(sanct_pos.0, sanct_pos.1, camera_pos.0, camera_pos.1);

    gui.x = x;
    gui.y = y;
}

fn is_sanct_visible(sanct_x: i32, sanct_y: i32, cam_x: i32, cam_y: i32) -> bool {
    collisions::are_overlapping(sanct_x, sanct_y, SANCTUARY_WIDTH as i32, SANCTUARY_HEIGHT as i32, 
                             cam_x, cam_y, CAMERA_DEFAULT_SIZE as i32, CAMERA_DEFAULT_SIZE as i32)
}

fn get_gui_pos(sanct_post_x: i32, sanct_post_y: i32, cam_x: i32, cam_y: i32) -> (f32, f32) {
    let pos_x_border = CAMERA_DEFAULT_SIZE/2.;
    let pos_y_border = pos_x_border;
    let neg_x_border = -pos_x_border;
    let neg_y_border = neg_x_border;

    let ux = (sanct_post_x - cam_x) as f32;
    let uy = (sanct_post_y - cam_y) as f32;

    let longueur = (ux.powi(2) + uy.powi(2)).sqrt();
    let ux = ux/longueur;
    let uy = uy/longueur;

    let gx = ux * pos_x_border;
    let gy = uy * pos_y_border;

    let x = if gx.abs() > pos_x_border {
        if gx > 0. {
            pos_x_border
        } else {
            neg_x_border
        }
    } else {
        gx
    };

    let y = if gy.abs() > pos_y_border {
        if gy > 0. {
            pos_y_border
        } else {
            neg_y_border
        }
    } else {
        gy
    };

    (x + cam_x as f32, y + cam_y as f32)
}

fn update_visibility(mut query: Query<(&mut Visibility, &GUI)>) {
    for (mut sprite_visibility, gui) in query.iter_mut() {
        *sprite_visibility = if gui.visible { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn update_display_pos(mut query: Query<(&mut Transform, &GUI)>) {
    for (mut transform, gui) in query.iter_mut() {
        transform.translation.x = gui.x;
        transform.translation.y = gui.y;
    }
}


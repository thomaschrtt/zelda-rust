mod player;
mod constants;
mod structures;
mod setup;
mod collisions;
mod gui;
mod ennemies;
mod entitypattern;
mod menu;
mod pause;
mod gameover;

use bevy::prelude::*;
use bevy::window::WindowMode;
use ennemies::EnnemyPlugin;
use gameover::GameOverPlugin;
use pause::PausePlugin;
use setup::SetupPlugin;
use crate::player::*;
use crate::structures::*;
use crate::gui::*;
use crate::constants::*;


#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default] Menu,
    Playing,
    Paused,
    GameOver,
}


fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, menu::MenuPlugin, PlayerPlugin, SetupPlugin, EnnemyPlugin, StructuresPlugin, GUIPlugin, PausePlugin, GameOverPlugin))
        .add_systems(Startup, setup_window)
        .run();
}

fn setup_window(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    
) {
    let mut window = windows.single_mut();
    window.mode = WindowMode::BorderlessFullscreen;
    window.title = "Zelda".to_string();

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale : CAMERA_DEFAULT_SCALE,
            far: Z_LAYER_GUI,
            ..OrthographicProjection::default()
        },
        ..Default::default()
    });
}
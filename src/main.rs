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
mod buttons;
mod loading;

use bevy::prelude::*;
use bevy::window::WindowMode;
use ennemies::EnnemyPlugin;
use gameover::GameOverPlugin;
use loading::RestartButtonPlugin;
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
    Loading,
}

#[derive(Resource, Default)]
struct LoadingState {
    timer: Timer,
}

#[derive(Resource)]
pub struct GameConfig {
    pub seed: u64,
}

impl GameConfig {
    pub fn new(seed: u64) -> Self {
        GameConfig { seed }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            seed: DEFAULT_SEED,
        }
    }
}
    

fn main() {
    let args: Vec<String> = std::env::args().collect();
    App::new()
        .insert_resource(if args.len() > 1 {
            GameConfig::new(args[1].parse().unwrap_or(DEFAULT_SEED))
        } else {
            GameConfig::default()
        })
        .insert_resource(LoadingState {
            timer: Timer::from_seconds(1.0, TimerMode::Once), // 1 seconde
        })
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins, 
            menu::MenuPlugin, 
            PlayerPlugin, 
            SetupPlugin, 
            EnnemyPlugin, 
            StructuresPlugin, 
            GUIPlugin, 
            PausePlugin, 
            GameOverPlugin, 
            buttons::ButtonPlugin,
            RestartButtonPlugin))
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
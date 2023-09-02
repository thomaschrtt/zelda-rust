mod player;
mod constants;
mod structures;
mod setup;
mod collisions;
mod gui;
mod ennemies;
mod entitypattern;

use bevy::prelude::*;
use ennemies::EnnemyPlugin;
use crate::player::*;
use crate::structures::*;
use crate::gui::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, StructuresPlugin, GUIPlugin, EnnemyPlugin))
        .add_systems(Startup, (setup::setup, setup::setup_random_trees, setup::setup_random_bushes, setup::setup_random_graves))
        .add_systems(Update, (setup::zoom_camera, setup::track_player,))
        .run();
}
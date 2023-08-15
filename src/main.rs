mod player;
mod constants;
mod structures;
mod setup;
mod collisions;
mod gui;

use bevy::prelude::*;
use crate::player::*;
use crate::structures::*;
use crate::gui::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, StructuresPlugin, GUIPlugin))
        .add_systems(Startup, (setup::setup, setup::setup_random_trees, setup::setup_random_bushes))
        .add_systems(Update, (setup::zoom_camera, setup::track_player))
        .run();
}
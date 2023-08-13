mod player;
mod constants;
mod structures;
mod setup;
mod collisions;
mod gui;

use bevy::prelude::*;
use crate::player::*;
use crate::structures::*;
use crate::collisions::*;
use crate::gui::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, StructuresPlugin, CollisionPlugin, GUIPlugin))
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (setup::zoom_camera, setup::track_player))
        .run();
}
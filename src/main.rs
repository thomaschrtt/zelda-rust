mod player;
mod constants;
mod structures;
mod setup;
mod collisions;

use bevy::prelude::*;
use crate::player::*;
use crate::structures::*;
use crate::collisions::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, StructuresPlugin, CollisionPlugin))
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (setup::zoom_camera, setup::track_player))
        .run();
}
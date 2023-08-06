mod player;
mod constants;
mod structures;
mod setup;

use bevy::prelude::*;
use crate::player::*;
use crate::constants::*;
use crate::setup::*;
use crate::structures::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, StructuresPlugin))
        .add_systems(Startup, setup::setup)
        .run();
}
mod player;
mod constants;
mod structures;
mod setup;

use bevy::prelude::*;
use crate::player::*;
use crate::constants::*;
use crate::setup::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup::setup)
        .run();
}
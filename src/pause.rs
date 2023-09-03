use bevy::prelude::*;

use crate::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_pause.run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))));
    }
}

fn show_pause(
    keyinput: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyinput.just_pressed(KeyCode::Escape) {
        state.set(if current_state.get().eq(&GameState::Playing) {
            GameState::Paused
        } else {
            GameState::Playing
        });
    }

}
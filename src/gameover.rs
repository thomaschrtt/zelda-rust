use bevy::{prelude::*, app::AppExit};

use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_gameover);
    }
}

fn setup_gameover(
    mut app_exit_events: EventWriter<AppExit>,
)
{
    app_exit_events.send(AppExit);
}
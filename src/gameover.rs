use bevy::{prelude::*, app::AppExit};

use crate::{GameState, buttons::create_button, loading::create_restart_button};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_gameover)
            .add_systems(Update, interact_with_quit_button);
    }
}

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct QuitButton;

fn setup_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Column,

                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),

                ..default()
            },
            ..default()
        }, GameOver))
        .with_children(|parent| {
            create_restart_button(parent, &asset_server)
        })
        .with_children(|parent| {
            create_button(parent, "Quit", QuitButton, &asset_server)
        });
}

fn interact_with_quit_button(
    mut button_query: Query<(&Interaction, &QuitButton)>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, _) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                exit.send(AppExit);
            }
            _ => {}
        }
    }
}



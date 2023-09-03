use bevy::{prelude::*, app::AppExit};

use crate::{GameState, buttons::create_button};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), pause_menu)
        .add_systems(Update, show_pause.run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))))
        .add_systems(Update, (quit, resume).run_if(in_state(GameState::Paused)));
    }
}

fn show_pause(
    keyinput: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    mut commands: Commands,
    pause_query: Query<Entity, With<Pause>>
) {
    if keyinput.just_pressed(KeyCode::Escape) {
        state.set(if current_state.get().eq(&GameState::Playing) {
            GameState::Paused
        } else {
            for entity in pause_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            GameState::Playing
        });
    }
}

#[derive(Component)]
pub struct Pause;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct ResumeButton;

fn pause_menu(
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
        }, Pause))
        .with_children(|parent| {
            create_button(parent, "Resume", ResumeButton, &asset_server)
        })
        .with_children(|parent| {
            create_button(parent, "Quit", QuitButton, &asset_server)
        });
}

fn quit(
    mut button_query: Query<(&Interaction, &QuitButton)>,
    mut exit: ResMut<Events<AppExit>>,
) {
    for (interaction, _) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit);
        }
    }
}

fn resume(
    mut button_query: Query<(&Interaction, &ResumeButton)>,
    mut state: ResMut<NextState<GameState>>,
    pause_query: Query<Entity, With<Pause>>,
    mut commands: Commands,
) {
    for (interaction, _) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            state.set(GameState::Playing);
            for entity in pause_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

        }
    }
}

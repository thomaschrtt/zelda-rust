use bevy::{prelude::*, app::AppExit};

use crate::{GameState, buttons::create_button, ennemies::Ennemy, player::Player, setup::{BackgroundObjects, Background}, structures::{Sanctuary, Tower}, gui::{GUI, self}, LoadingState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_gameover)
           .add_systems(Update, (interact_with_quit_button, interact_with_restart_button).run_if(in_state(GameState::GameOver)))
           .add_systems(Update, load_game.run_if(in_state(GameState::Loading)));
    }
}

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct RestartButton;

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
            create_button(parent, "Restart", RestartButton, &asset_server)
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

fn interact_with_restart_button(
    mut button_query: Query<(&Interaction, &RestartButton)>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameover_query: Query<Entity, With<GameOver>>,
    entity_query: Query<Entity, With<Ennemy>>,
    player_query: Query<Entity, With<Player>>,
    bg_element_query: Query<Entity, With<BackgroundObjects>>,
    sanctuary_query: Query<Entity, With<Sanctuary>>,
    tower_query: Query<Entity, With<Tower>>,
    gui_query: Query<Entity, With<GUI>>,
    bg_query: Query<Entity, With<Background>>,
) {
    for (interaction, _) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for entity in gameover_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                despawn_everything(&mut commands,
                                      &asset_server, 
                                   &entity_query, 
                                   &player_query, 
                                   &bg_element_query,
                                   &sanctuary_query,
                                   &tower_query,
                                   &gui_query,
                                   &bg_query);
                state.set(GameState::Loading);
                
            }
            _ => {}
        }
    }
}

fn despawn_everything(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_query: &Query<Entity, With<Ennemy>>,
    player_query: &Query<Entity, With<Player>>,
    bg_element_query: &Query<Entity, With<BackgroundObjects>>,
    sanctuary_query: &Query<Entity, With<Sanctuary>>,
    tower_query: &Query<Entity, With<Tower>>,
    gui_query: &Query<Entity, With<GUI>>,
    bg_query: &Query<Entity, With<Background>>,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in bg_element_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in sanctuary_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in tower_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in gui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in bg_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


fn load_game(
    mut state: ResMut<NextState<GameState>>,
    mut loading_state: ResMut<LoadingState>,
    time: Res<Time>,
) {
    loading_state.timer.tick(time.delta());

    if loading_state.timer.finished() {
        state.set(GameState::Playing);
        loading_state.timer.reset();
    }
}
use bevy::prelude::*;

use crate::{ennemies::Ennemy, player::Player, setup::{BackgroundObjects, Background}, structures::{Sanctuary, Tower}, gui::GUI, gameover::GameOver, GameState, LoadingState, buttons::create_button};


pub struct RestartButtonPlugin;

impl Plugin for RestartButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interact_with_restart_button)
            .add_systems(Update, load_game.run_if(in_state(GameState::Loading)));
    }
}


fn interact_with_restart_button(
    mut self_button: Query<(&Interaction, &RestartButton)>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    gameover_query: Query<Entity, With<GameOver>>,
    entity_query: Query<Entity, With<Ennemy>>,
    player_query: Query<Entity, With<Player>>,
    bg_element_query: Query<Entity, With<BackgroundObjects>>,
    sanctuary_query: Query<Entity, With<Sanctuary>>,
    tower_query: Query<Entity, With<Tower>>,
    gui_query: Query<Entity, With<GUI>>,
    bg_query: Query<Entity, With<Background>>,
    menu_node: Query<Entity, With<Node>>,
) {
    for (interaction, _) in self_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for entity in gameover_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                despawn_everything(&mut commands, 
                                   &entity_query, 
                                   &player_query, 
                                   &bg_element_query,
                                   &sanctuary_query,
                                   &tower_query,
                                   &gui_query,
                                   &bg_query,
                                    &menu_node);
                state.set(GameState::Loading);
                
            }
            _ => {}
        }
    }
}

fn despawn_everything(
    commands: &mut Commands,
    entity_query: &Query<Entity, With<Ennemy>>,
    player_query: &Query<Entity, With<Player>>,
    bg_element_query: &Query<Entity, With<BackgroundObjects>>,
    sanctuary_query: &Query<Entity, With<Sanctuary>>,
    tower_query: &Query<Entity, With<Tower>>,
    gui_query: &Query<Entity, With<GUI>>,
    bg_query: &Query<Entity, With<Background>>,
    button_query: &Query<Entity, With<Node>>,
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
    for entity in button_query.iter() {
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

#[derive(Component)]
pub struct RestartButton;

pub fn create_restart_button(
    commands: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    create_button(commands, "Restart", RestartButton, &asset_server)
}

use bevy::app::AppExit;
use bevy::prelude::*;
use crate::buttons::*;
use crate::constants::*;
use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (intteract_with_play_button, intteract_with_quit_button, start_on_press_space).run_if(in_state(GameState::Menu)));
    }
}

#[derive(Component)]
pub struct Menu;
#[derive(Component)]
pub struct ButtonPlay;
#[derive(Component)]
pub struct ButtonQuit;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., Z_LAYER_BACKGROUND),
            ..Transform::default()
        },
        ..Default::default()
    });

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
        }, Menu))
        .with_children(|parent| {
            create_button(parent, "Play", ButtonPlay, &asset_server)
        })
        .with_children(|parent| {
            create_button(parent, "Quit", ButtonQuit, &asset_server)
        });
}

fn intteract_with_play_button(
    mut state: ResMut<NextState<GameState>>,
    mut button_query: Query<(&Interaction, &ButtonPlay)>,
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>
) {
    for (interaction, _) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            state.set(GameState::Playing);
            despawn_menu(&mut commands, &menu_query)
        }
    }
}

fn intteract_with_quit_button(
    mut button_query: Query<(&Interaction, &ButtonQuit)>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, _) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit);
        }
    }
}

fn despawn_menu(commands: &mut Commands, menu_query: &Query<Entity, With<Menu>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn start_on_press_space(
    mut state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing);
        despawn_menu(&mut commands, &menu_query)
    }
}


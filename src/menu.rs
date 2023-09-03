use bevy::app::AppExit;
use bevy::{prelude::*, window::WindowMode};
use crate::constants::*;
use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (intteract_with_play_button, intteract_with_quit_button).run_if(in_state(GameState::Menu)));
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
            create_button(parent, "Play", ButtonPlay)
        })
        .with_children(|parent| {
            create_button(parent, "Quit", ButtonQuit)
        });
}

fn create_button<T: Component>(
    commands: &mut ChildBuilder,
    button_text: &str,
    button_component: T,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.9).into(),
            ..Default::default()
        })
        .insert(button_component)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: button_text.to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..Default::default()
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });
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

fn despawn_menu(mut commands: &mut Commands, menu_query: &Query<Entity, With<Menu>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


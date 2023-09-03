use bevy::prelude::*;

use crate::constants::{BUTTON_WIDTH, BUTTON_HEIGHT};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_animation);
    }
}

#[derive(Component)]
pub struct ButtonCompo;

pub fn create_button<T: Component>(
    commands: &mut ChildBuilder,
    button_text: &str,
    button_component: T,
    asset_server: &Res<AssetServer>, 
) {
    let texture: Handle<Image> = asset_server.load("button.png");
    
    commands
        .spawn((ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: UiImage::new(texture),
            ..Default::default()
        }, ButtonCompo))
        .insert(button_component)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: button_text.to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::GRAY,
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

fn hover_animation(
    mut button_query: Query<(&Interaction, &mut Style), With<ButtonCompo>>,
) {
    for (interaction, mut style) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                style.width = Val::Px(BUTTON_WIDTH * 0.9);
                style.height = Val::Px(BUTTON_HEIGHT * 0.9);
            }
            Interaction::Hovered => {
                style.width = Val::Px(BUTTON_WIDTH * 1.1);
                style.height = Val::Px(BUTTON_HEIGHT * 1.1);
            }
            Interaction::None => {
                style.width = Val::Px(BUTTON_WIDTH);
                style.height = Val::Px(BUTTON_HEIGHT);
            }
        }
    }
}
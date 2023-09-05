use bevy::prelude::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_animation);
    }
}

#[derive(Component)]
pub struct ButtonCompo;

#[derive(Component)]
pub struct ButtonOriginalSize {
    pub width: f32,
    pub height: f32,
}

pub fn create_button<T: Component>(
    commands: &mut ChildBuilder,
    button_text: &str,
    button_component: T,
    asset_server: &Res<AssetServer>, 
) {
    let texture: Handle<Image> = asset_server.load("UI/button.png");
    
    commands
        .spawn((ButtonBundle {
            style: Style {
                width: Val::Auto,
                max_width: Val::Px(400.),
                min_height: Val::Px(100.0),
                padding: UiRect::all(Val::Px(20.0)),
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
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn button_animation(
    mut button_query: Query<(&Interaction, &mut Style, &mut UiImage), With<ButtonCompo>>,
    asset_server: Res<AssetServer>,
) {
    let not_pressed: Handle<Image> = asset_server.load("UI/button.png");
    let pressed: Handle<Image> = asset_server.load("UI/pressed_button.png");
    for (interaction, mut style, mut image) in button_query.iter_mut() {

        match *interaction {
            Interaction::Pressed => {
                *image = UiImage::new(pressed.clone());
            }
            Interaction::Hovered => {
                style.padding = UiRect::axes(Val::Px(10.0), Val::Px(20.0));
            }
            Interaction::None => {
                *image = UiImage::new(not_pressed.clone());
                style.padding = UiRect::all(Val::Px(20.0));
            }
        }
    }
}
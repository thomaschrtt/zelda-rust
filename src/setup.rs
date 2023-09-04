use bevy::prelude::*;
use rand::prelude::*;
use crate::{constants::*, player::*, collisions::{CollisionComponent, Collisionable}, GameState, GameConfig};


pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Menu), (setup, 
                                                    setup_random_trees, 
                                                    setup_random_bushes, 
                                                    setup_random_graves, ))
            .add_systems(Update, (
                // zoom_camera, 
                                                   track_player,
                                                   ).run_if(in_state(GameState::Playing)));
    }
}


pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., Z_LAYER_BACKGROUND),
            ..Transform::default()
        },
        ..Default::default()
    });

}

// pub fn zoom_camera(
//     mut query: Query<&mut OrthographicProjection>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     let mut transform = query.single_mut();
//     if keyboard_input.pressed(KeyCode::S) && transform.scale < CAMERA_MAX_SCALE {
//         transform.scale = transform.scale + 0.01;
//     }
//     if keyboard_input.pressed(KeyCode::Z) && transform.scale > CAMERA_MIN_SCALE{
//         transform.scale = transform.scale - 0.01;
//     }
//     if keyboard_input.pressed(KeyCode::R) {
//         transform.scale = CAMERA_DEFAULT_SCALE;
//     }
//     if keyboard_input.pressed(KeyCode::M) {
//         transform.scale = CAMERA_MAX_SCALE;
//     }

// }

pub fn track_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Camera, &mut Transform), Without<Player>>,
    mut camera_proj : Query<&mut OrthographicProjection, With<Camera>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera.single_mut().1;
    let camera_projection = camera_proj.single_mut();

    let camera_range_height = camera_projection.scale * WINDOW_HEIGHT / 2.;
    let camera_range_width = camera_projection.scale * WINDOW_WIDTH / 2.;
    
    let x = player_transform.translation.x;
    let y = player_transform.translation.y;

    let camera_max_x = MAP_SIZE / 2. - camera_range_width;
    let camera_min_x = -MAP_SIZE / 2. + camera_range_width;
    let camera_max_y = MAP_SIZE / 2. - camera_range_height;
    let camera_min_y = -MAP_SIZE / 2. + camera_range_height;

    camera_transform.translation.x = if x > camera_max_x { camera_max_x } else if x < camera_min_x { camera_min_x } else { x };
    camera_transform.translation.y = if y > camera_max_y { camera_max_y } else if y < camera_min_y { camera_min_y } else { y };
}


pub enum BackgroundObjectType {
    Tree,
    Bush,
    BigGrave,
    SmallGrave,
    Bench,
}

#[derive(Component)]
pub struct BackgroundObjects {
    obj_type: BackgroundObjectType,
}

impl BackgroundObjects {
    pub fn get_type(&self) -> &BackgroundObjectType {
        &self.obj_type
    }
}

pub fn setup_random_trees(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    collisionable_query: Query<&CollisionComponent>,
    game_config: Res<GameConfig>,
) {
    let tree_texture_handle = asset_server.load("trees.png");
    let tree_texture_atlas = TextureAtlas::from_grid(tree_texture_handle, Vec2::new(TREE_WIDTH, TREE_HEIGHT), 3, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(3., 0.)));
    let tree_texture_atlas_handle = texture_atlases.add(tree_texture_atlas);

    let mut rng = StdRng::seed_from_u64(game_config.seed + OFFSET_TREE);

    for _ in 0..TREE_NUMBER {
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
            y = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
            let collisioncomponent = CollisionComponent::new(x, y- TREE_HEIGHT/2. + 12., 5., 5.);
            if !collisionable_query.iter().any(|collisionable| collisioncomponent.would_collide_with(collisionable)) {
                break;
            }
        }
        let index = rng.gen_range(0..3);

        
        let collisioncomponent = CollisionComponent::new(x, y- TREE_HEIGHT/2. + 12., 5., 5.);

        commands.spawn(SpriteSheetBundle {
            texture_atlas: tree_texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, -y+MAP_SIZE/2. + TREE_HEIGHT/2. - 12.),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        }).insert(BackgroundObjects { obj_type: BackgroundObjectType::Tree })
        .insert(collisioncomponent);
    }
}

pub fn setup_random_bushes(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_config: Res<GameConfig>,
) {
    let bush_texture_handle = asset_server.load("bushes.png");
    let bush_texture_atlas = TextureAtlas::from_grid(bush_texture_handle, Vec2::new(BUSH_WIDTH, BUSH_HEIGHT), 3, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let bush_texture_atlas_handle = texture_atlases.add(bush_texture_atlas);

    let mut rng = StdRng::seed_from_u64(game_config.seed + OFFSET_BUSH );
    for _ in 0..BUSH_NUMBER {
        let x = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        let y = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        let index = rng.gen_range(0..3);
        commands.spawn(SpriteSheetBundle {
            texture_atlas: bush_texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, -y+MAP_SIZE/2.),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(BackgroundObjects { obj_type: BackgroundObjectType::Bush });
    }
}

pub fn setup_random_graves(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    collisionable_query: Query<&CollisionComponent>,
    game_config: Res<GameConfig>,
) {
    let big_grave_texture_handle = asset_server.load("graves.png");
    let big_grave_texture_atlas = TextureAtlas::from_grid(big_grave_texture_handle, Vec2::new(64., 64.), 3, 1, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let big_grave_texture_atlas_handle = texture_atlases.add(big_grave_texture_atlas);

    let mut rng = StdRng::seed_from_u64(game_config.seed + OFFSET_GRAVE);
    for _ in 0..GRAVES_NUMBER {
        let mut x = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        let mut y = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        let mut collisioncomponent: CollisionComponent;
        let index = rng.gen_range(0..3);
        loop {
            let w = match index {
                0 => 32.,
                1 => 30.,
                2 => 56.,
                _ => 32.,
            };

            let h = match index {
                0 => 57.,
                1 => 41.,
                2 => 40.,
                _ => 40.,
            };

            collisioncomponent = CollisionComponent::new(x , y, w, h);
            if !collisionable_query.iter().any(|collisionable| collisioncomponent.would_collide_with(collisionable)) {
                break;
            }
            x = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
            y = rng.gen_range(-MAP_SIZE / 2. + 32.0..MAP_SIZE / 2. - 32.);
        }

        commands.spawn(SpriteSheetBundle {
            texture_atlas: big_grave_texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, -y+MAP_SIZE/2.),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .insert(BackgroundObjects { obj_type: match index {
            0 => BackgroundObjectType::BigGrave,
            1 => BackgroundObjectType::SmallGrave,
            2 => BackgroundObjectType::Bench,
            _ => BackgroundObjectType::BigGrave,
        } })
        .insert(collisioncomponent);
    }
}

// pub fn show_collisionable_component(
//     mut collisions_compo: Query<&CollisionComponent>
//     , mut commands: Commands
// ) {
//     for element in collisions_compo.iter_mut() {
//         let (x,y, w, h) = element.get_hitbox();
//         commands.spawn(SpriteBundle {
//             transform: Transform {
//                 translation: Vec3::new(x, y, Z_LAYER_GUI),
//                 ..Transform::default()
//             },
//             sprite: Sprite {
//                 color: Color::rgb(1., 0., 0.),
//                 custom_size: Some(Vec2::new(w, h)),
//                 ..Default::default()
//             },
//             ..Default::default()
//         });
//     }
// }

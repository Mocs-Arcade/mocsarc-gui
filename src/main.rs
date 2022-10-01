use bevy::{prelude::*, window::{PresentMode, WindowMode}, asset::AssetServerSettings };
use lerp::Lerp;
use serde::{Deserialize};
use toml::de::Error;
use std::fs;

const CLEAR : Color = Color::rgb(0.1,0.1,0.1);

// includes the file "game_listing.rs"
mod game_listing;
use game_listing::*;

mod selector;
use selector::*;

fn main () {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 1920.,
            height: 1080.,
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        //Ensure that bevy's AssetServer points to our root directory instead of "./assets"
        .insert_resource(AssetServerSettings {
            asset_folder: "".to_string(),
            ..default()
        })
        .init_resource::<Selector>()
        .insert_resource(Selector::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_listings)
        .add_system(select)
        .add_system(lerp_camera)
        .add_startup_system(load_assets)
        .run();
}

fn load_assets (
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {

    let font = asset_server.load("assets/game-played-font/GamePlayed-vYL7.ttf");
    let text_style = TextStyle {
        font,
        font_size: 150.0,
        color: Color::WHITE,
    };

    commands
    .spawn_bundle(
        Text2dBundle {
            text: Text::from_section("Mocs Arcade", text_style)
                .with_alignment(TextAlignment::TOP_CENTER),
            transform: Transform::from_xyz(0.0, 450.0, 0.0),
            ..default()
        }
    );

    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            align_content: AlignContent::Center,
            size: Size::new(Val::Px(1920.0), Val::Px(180.0)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    })
    .with_children(|parent| {
        // bevy logo (image)
        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                ..default()
            },
            image: asset_server.load("assets/GUI_Banner.png").into(),
            ..default()
        });
    });
}

fn load_listings (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut selector: ResMut<Selector>
) { 
    // Get all the filepaths to the folders in "./games"
    let paths = fs::read_dir("games\\").unwrap();
    // ID variable to iterate.
    let mut id_count: u32 = 0;

    //Iterate through the paths and load the listing file in the folder.
    for path in paths {
        //let cfg_data = fs::read_to_string(path.as_ref().unwrap().path().join("listing.toml"));
        let cfg_data : String = match fs::read_to_string(path.as_ref().unwrap().path().join("listing.toml")) {
            Ok(s) => {
                let deserialized_data: GameListing = match toml::from_str::<GameListing>(&s) {
                    Ok(gl) => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: asset_server.load(path.unwrap().path().join(&gl.config.img_path)),
                            transform: Transform::from_xyz((-336.0*2.0) + (((id_count as f32)%5.0) * 336.0), (((id_count/5) as f32)) * -478.0, 0.0),
                            ..default()
                        })
                            .insert(ID(id_count))
                            .insert(gl);
                        id_count += 1;
                        selector.total_games = id_count + 1;
                        GameListing::default()
                    },
                    Err(error) => {println!("Error parsing the listing.toml : {:?}", error.to_string()); GameListing::default()},
                };
                s
            },
            Err(error) => {println!("Error loading the listing.toml : {:?}", error.to_string()); String::default()},
        };
    }

    commands.spawn_bundle(Camera2dBundle::default());
}

const SCALE_SMOOTHING : f32 = 0.35;
const TRANSLATE_SMOOTHING : f32 = 0.15;

fn select (
    mut res: ResMut<Selector>,
    mut query_games: Query<(&mut GameListing, &ID, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    // Joystick up
    if keys.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        if res.current > 4 { res.current -= 5; }
    }
    // Joystick Down
    if keys.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        if res.current + 5 < res.total_games - 1 { res.current += 5; }
    }
    // Joystick Left
    if keys.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        if res.current != 0 { res.current -= 1; }
    }
    // Joystick Right
    if keys.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        if res.current + 1 < res.total_games - 1 { res.current += 1; }
    }
    // A down
    if keys.any_just_pressed([KeyCode::J, KeyCode::E]) {
        
    }
    // B down
    if keys.any_just_pressed([KeyCode::K, KeyCode::R]) {
        println!("B DOWN");
    }

    for (gl, id, mut tr) in &mut query_games {
        if id.0 == res.current {
            res.y_value = tr.translation.y;
            let mut scale = tr.scale.x.lerp(1.15, SCALE_SMOOTHING);
            tr.translation = Vec3::new(tr.translation.x, tr.translation.y, 100.0);
            tr.scale = Vec3::new(scale,scale,scale);
        } else {
            let mut scale = tr.scale.x.lerp(1.0, SCALE_SMOOTHING);
            tr.translation = Vec3::new(tr.translation.x, tr.translation.y, 0.0);
            tr.scale = Vec3::new(scale,scale,scale);
        }
    }

    if keys.pressed(KeyCode::Space) {
        println!("{}", res.current);
    }
    
}

fn lerp_camera (
    mut res: ResMut<Selector>,
    mut query_cam: Query<(&mut Transform, With<Camera>)>,
) {
    for (mut tr, o) in &mut query_cam {
        tr.translation.y = tr.translation.y.lerp(res.y_value - 50.0,TRANSLATE_SMOOTHING);
    }
}

fn query_games (
    query : Query<&GameListing>
) {
    for game in &query {
        println!("{:?}", game)
    }
}

fn keyboard_input (
    keys: Res<Input<KeyCode>>,
) {
    // Joystick up
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        println!("JOYSTICK UP");
    }
    // Joystick Down
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        println!("JOYSTICK DOWN");
    }
    // Joystick Left
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        println!("JOYSTICK LEFT");
    }
    // Joystick Right
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        // Either the left or right shift are being held down
        println!("JOYSTICK RIGHT");
    }
    // A down
    if keys.any_pressed([KeyCode::J, KeyCode::E]) {
        println!("A DOWN");
    }
    // B down
    if keys.any_pressed([KeyCode::K, KeyCode::R]) {
        println!("B DOWN");
    }
}
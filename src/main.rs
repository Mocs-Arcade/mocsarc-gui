
/// - Bevy Includes - ///
use bevy::{prelude::*, window::{PresentMode, WindowMode}, asset::AssetServerSettings};
use serde::Deserialize;

/// Toml Includes ///
//use toml::de::Error;

/// STL Includes ///
use std::{fs, path::PathBuf};

/// - Lerp Includes - ///
use lerp::Lerp;

/// - Local Includes - ///
mod game_listing;
use game_listing::{GameListing, ID};

mod selector;
use selector::{Selector, select, lerp_camera};

mod download;
use download::*;

// Configuration Constant - the background color //
const CLEAR : Color = Color::rgb(0.1,0.1,0.1);

fn main () {

    download();

    App::new()
        // Tell bevy to clear the screen with Black
        .insert_resource(ClearColor(CLEAR))
        // Set up the window
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
        // done for convenience
        .insert_resource(AssetServerSettings {
            asset_folder: "".to_string(),
            ..default()
        })
        // Initialize our Selector resource so it can be accessed
        .init_resource::<Selector>()
        .insert_resource(Selector::default())
        // Default plugins will add everything we need
        .add_plugins(DefaultPlugins)
        // Add all of our systems we want to run
        .add_startup_system(load_listings)
        .add_system(select)
        .add_system(lerp_camera)
        .add_startup_system(load_assets)
        .run();
}

/// Load Static Assets into the GUI
fn load_assets (
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {

    // load text for top of screen

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
    /* unused for now
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
    */
}

/// Load all games into the Bevy World
fn load_listings (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut selector: ResMut<Selector>
) { 
    // Get all the filepaths to the folders in "./games"
    let paths = fs::read_dir("games\\").unwrap();
    // ID variable to iterate.
    let mut id_count: u32 = 0;

    // Iterate through the paths and load the listing file in the folder.
    for path in paths {
        let _cfg_data : String = match fs::read_to_string(path.as_ref().unwrap().path().join("listing.toml")) {
            Ok(s) => {
                let _deserialized_data: GameListing = match toml::from_str::<GameListing>(&s) {
                    Ok(gl) => {
                        // Check to see if the cover image path is valid
                        let mut img_path : PathBuf = path.unwrap().path().join(&gl.config.img_path);
                        if !img_path.exists() {
                            img_path = PathBuf::from("assets/img_error.png");
                        }

                        commands.spawn_bundle(SpriteBundle {
                            texture: asset_server.load(img_path),
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

/*
fn query_games (
    query : Query<&GameListing>
) {
    for game in &query {
        println!("{:?}", game)
    }
}
*/

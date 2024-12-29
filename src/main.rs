use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::reflect::TypePath;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;

pub mod system;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 60.0;

#[derive(serde::Deserialize, Asset, TypePath)]
struct SaveFile {
    number: i32,
}

#[derive(Resource)]
struct SaveFileAsset(Handle<SaveFile>);

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "flower".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
        FrameTimeDiagnosticsPlugin,
        JsonAssetPlugin::<SaveFile>::new(&["save.json"]),
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(TICK_TIME));
    app.add_systems(Startup, setup);
    app.add_systems(FixedUpdate, system::kill_game_on_esc);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
    load_level(commands, asset_server);
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = SaveFileAsset(asset_server.load("save1.save.json"));
    commands.insert_resource(handle);
}

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;

pub mod flower;
pub mod input;
pub mod map;
pub mod save;
pub mod system;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 60.0;

#[derive(Component)]
struct Clickable;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingScreen,
    MainMenu,
    Overview,
    ShowFlower,
}

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
        JsonAssetPlugin::<save::SaveFile>::new(&["save.json"]),
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(TICK_TIME));
    app.insert_state(GameState::Overview);
    app.add_systems(Startup, setup);
    app.add_systems(
        FixedUpdate,
        (system::kill_game_on_esc, input::input_mouse_button),
    );
    app.add_systems(OnEnter(GameState::Overview), map::create_map);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_http_client::prelude::*;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

pub mod flower;
pub mod http;
pub mod input;
pub mod map;
pub mod save;
pub mod system;
pub mod ui;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 60.0;

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct PlayerInfo {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub token: String,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingScreen,
    Login,
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
        TextInputPlugin,
        HttpClientPlugin,
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(TICK_TIME));
    app.insert_state(GameState::Login);
    app.add_systems(Startup, setup);
    app.add_systems(
        FixedUpdate,
        (
            system::kill_game_on_esc,
            input::input_mouse_button,
            ui::text_update_time,
            http::handle_response_login,
        ),
    );
    app.add_systems(Update, (ui::text_input_listener.after(TextInputSystem),));
    app.add_systems(OnEnter(GameState::Login), ui::create_login_ui);
    app.add_systems(OnEnter(GameState::Overview), map::create_map);
    app.register_request_type::<http::LoginData>();
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let time = chrono::Utc::now();

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 15.0,
        ..default()
    };
    let text_justification = JustifyText::Left;

    commands.spawn(Camera2d);

    commands.spawn(PlayerInfo {
        id: 1,
        name: "".to_string(),
        password: "".to_string(),
        token: "".to_string(),
    });

    commands.spawn((
        Text2d::new(time.format("%Y-%m-%d %H:%M:%S").to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_translation(Vec3::new(0.0, SCREEN_HEIGHT / 2.0 - 15.0, 0.0)),
        ui::DateTimeText,
    ));
}

use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod system;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 60.0;

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
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(TICK_TIME));
    app.add_systems(Startup, setup);
    app.add_systems(FixedUpdate, system::kill_game_on_esc);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

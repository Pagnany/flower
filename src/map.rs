use bevy::prelude::*;

pub fn create_map(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}

use crate::flower;
use bevy::prelude::*;

pub fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(-200.0, 0.0, 0.0),
        crate::Clickable,
        flower::Flower {
            id: 1,
            name: "Rose".to_string(),
            flower_type: "Red".to_string(),
            px_height: 50.0,
            px_width: 50.0,
        },
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        crate::Clickable,
        flower::Flower {
            id: 2,
            name: "Rose".to_string(),
            flower_type: "Red".to_string(),
            px_height: 50.0,
            px_width: 50.0,
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        crate::Clickable,
        flower::Flower {
            id: 3,
            name: "Rose".to_string(),
            flower_type: "Red".to_string(),
            px_height: 50.0,
            px_width: 50.0,
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(100.0, 0.0, 0.0),
        crate::Clickable,
        flower::Flower {
            id: 4,
            name: "Rose".to_string(),
            flower_type: "Red".to_string(),
            px_height: 50.0,
            px_width: 50.0,
        },
    ));
}

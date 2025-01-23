use crate::flower;
use crate::ui_overview::OverviewClickable;
use bevy::prelude::*;

pub fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(-200.0, 60.0, 0.0),
        OverviewClickable::Flower(1),
        flower::Flower {
            id: 1,
            name: "Rose".to_string(),
            flower_type: "Red".to_string(),
            ..Default::default()
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(-100.0, -90.0, 0.0),
        OverviewClickable::Flower(2),
        flower::Flower {
            id: 2,
            name: "Lily".to_string(),
            flower_type: "White".to_string(),
            ..Default::default()
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        OverviewClickable::Flower(3),
        flower::Flower {
            id: 3,
            name: "Carnation".to_string(),
            flower_type: "Pink".to_string(),
            ..Default::default()
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(100.0, -60.0, 0.0),
        OverviewClickable::Flower(4),
        flower::Flower {
            id: 4,
            name: "Cornflower".to_string(),
            flower_type: "Blue".to_string(),
            ..Default::default()
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("r.png")),
        Transform::from_xyz(200.0, 50.0, 0.0),
        OverviewClickable::Flower(5),
        flower::Flower {
            id: 5,
            name: "Wheat".to_string(),
            flower_type: "Brown".to_string(),
            ..Default::default()
        },
    ));
}

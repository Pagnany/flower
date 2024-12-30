use bevy::prelude::*;
use bevy::reflect::TypePath;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct SaveFile {
    pub number: i32,
}

#[derive(Resource)]
pub struct SaveFileAsset(pub Handle<SaveFile>);

pub fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = SaveFileAsset(asset_server.load("save1.save.json"));
    commands.insert_resource(handle);
}

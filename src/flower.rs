use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Flower {
    pub id: i32,
    pub name: String,
    pub flower_type: String,
    pub px_height: f32,
    pub px_width: f32,
}

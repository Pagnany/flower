use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Flower {
    pub id: i32,
    pub name: String,
    pub flower_type: String,
    pub px_height: f32,
    pub px_width: f32,
    pub image: String,
}

impl Default for Flower {
    fn default() -> Self {
        Flower {
            id: 0,
            name: "".to_string(),
            flower_type: "".to_string(),
            px_height: 50.0,
            px_width: 50.0,
            image: "".to_string(),
        }
    }
}

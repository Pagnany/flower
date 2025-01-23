use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Flower {
    pub id: i32,
    pub name: String,
    pub flower_type: String,
    pub image: String,
}

impl Default for Flower {
    fn default() -> Self {
        Flower {
            id: 0,
            name: "".to_string(),
            flower_type: "".to_string(),
            image: "".to_string(),
        }
    }
}

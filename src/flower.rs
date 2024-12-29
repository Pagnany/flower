use bevy::prelude::*;

#[derive(Component)]
pub struct Flower {
    pub id: i32,
    pub name: String,
    pub flower_type: String,
}

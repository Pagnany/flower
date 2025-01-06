use bevy::prelude::*;
pub fn text_update_time(mut text_query: Query<&mut Text2d, With<Text2d>>) {
    let time = chrono::Utc::now();
    let mut text = text_query.single_mut();
    *text = Text2d::new(time.format("%Y-%m-%d %H:%M:%S").to_string());
}

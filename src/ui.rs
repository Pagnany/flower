use bevy::prelude::*;

#[derive(Component)]
pub struct DateTimeText;

pub fn text_update_time(mut text_query: Query<&mut Text2d, With<DateTimeText>>) {
    let time = chrono::Utc::now();
    let mut text = text_query.single_mut();
    *text = Text2d::new(time.format("%Y-%m-%d %H:%M:%S").to_string());
}

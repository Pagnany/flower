use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputSubmitEvent};

#[derive(Component)]
pub struct DateTimeText;

pub fn text_update_time(mut text_query: Query<&mut Text2d, With<DateTimeText>>) {
    let time = chrono::Utc::now();
    let mut text = text_query.single_mut();
    *text = Text2d::new(time.format("%Y-%m-%d %H:%M:%S").to_string());
}

pub fn create_login_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };
    let text_justification = JustifyText::Left;

    commands.spawn((
        Text2d::new("Login".to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_translation(Vec3::new(0.0, crate::SCREEN_HEIGHT / 2.0 - 150.0, 0.0)),
    ));

    commands.spawn((
        Node {
            left: Val::Px(crate::SCREEN_WIDTH / 2.0 - 150.),
            top: Val::Px(200.),
            width: Val::Px(300.),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        TextInput,
    ));
}

pub fn text_input_listener(mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
    }
}

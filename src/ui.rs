use bevy::prelude::*;
use bevy_http_client::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputSubmitEvent};

#[derive(Component)]
pub struct DateTimeText;

#[derive(Component)]
pub struct LoginTextName;

#[derive(Component)]
pub struct LoginCaption;

pub fn text_update_time(mut text_query: Query<&mut Text2d, With<DateTimeText>>) {
    let time = chrono::Utc::now();
    let mut text = text_query.single_mut();
    *text = Text2d::new(time.format("%Y-%m-%d %H:%M:%S").to_string());
}

pub fn destroy_login_ui(
    mut commands: Commands,
    q: Query<Entity, With<LoginCaption>>,
    q2: Query<Entity, With<LoginTextName>>,
    q3: Query<Entity, With<TextInput>>,
) {
    q.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
    q2.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
    q3.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
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
        Transform::from_translation(Vec3::new(0.0, crate::SCREEN_HEIGHT / 2.0 - 350.0, 0.0)),
        LoginCaption,
    ));

    commands.spawn((
        Text2d::new("".to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_translation(Vec3::new(0.0, crate::SCREEN_HEIGHT / 2.0 - 150.0, 0.0)),
        LoginTextName,
    ));

    commands.spawn((
        Node {
            left: Val::Px(crate::SCREEN_WIDTH / 2.0 - 150.0),
            top: Val::Px(400.),
            width: Val::Px(300.),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        TextInput,
    ));
}

pub fn text_input_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut player_info: ResMut<crate::PlayerInfo>,
    mut q_text_name: Query<&mut Text2d, (With<LoginTextName>, Without<LoginCaption>)>,
    mut q_text_caption: Query<&mut Text2d, (With<LoginCaption>, Without<LoginTextName>)>,
    mut ev_request: EventWriter<TypedRequest<crate::http::LoginData>>,
) {
    for event in events.read() {
        if !event.value.is_empty() {
            if player_info.name.is_empty() {
                player_info.name = event.value.clone();

                let mut text = q_text_name.single_mut();
                *text = Text2d::new(player_info.name.clone());

                let mut text = q_text_caption.single_mut();
                *text = Text2d::new("Password".to_string());
            } else {
                player_info.password = event.value.clone();

                // request token from server
                let login = crate::http::Login {
                    username: player_info.name.clone(),
                    password: player_info.password.clone(),
                };
                ev_request.send(
                    HttpClient::new()
                        .post("https://pagnany.de/flower-api.php")
                        .json(&login)
                        .with_type::<crate::http::LoginData>(),
                );
            }
        }
    }
}

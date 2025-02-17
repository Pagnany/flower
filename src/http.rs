use bevy::prelude::*;
use bevy_http_client::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoginData {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Login {
    pub action: String,
    pub username: String,
    pub password: String,
}

pub fn send_request_login(mut ev_request: EventWriter<TypedRequest<LoginData>>) {
    let login = Login {
        action: "login".to_string(),
        username: "Pagnany".to_string(),
        password: "test".to_string(),
    };
    ev_request.send(
        HttpClient::new()
            .post("https://pagnany.de/flower-api.php")
            .json(&login)
            .with_type::<LoginData>(),
    );
}

pub fn handle_response_login(
    mut ev_response: EventReader<TypedResponse<LoginData>>,
    mut player_info: ResMut<crate::PlayerInfo>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for response in ev_response.read() {
        player_info.id = response.id.parse().unwrap();
        player_info.token = response.token.clone();

        next_state.set(crate::GameState::Overview);
    }
}

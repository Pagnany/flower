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
    pub username: String,
    pub password: String,
}

pub fn send_request_login(mut ev_request: EventWriter<TypedRequest<LoginData>>) {
    let login = Login {
        username: "Pagnany".to_string(),
        password: "test".to_string(),
    };
    ev_request.send(
        HttpClient::new()
            .post("http://api.pagnany.de")
            .json(&login)
            .with_type::<LoginData>(),
    );
}

pub fn handle_response_login(
    mut ev_response: EventReader<TypedResponse<LoginData>>,
    mut q_player_info: Query<&mut crate::PlayerInfo>,
) {
    for response in ev_response.read() {
        println!("ID: {}", response.id);
        println!("Token: {}", response.token);

        let mut player_info = q_player_info.single_mut();
        player_info.id = response.id.parse().unwrap();
        player_info.token = response.token.clone();
    }
}

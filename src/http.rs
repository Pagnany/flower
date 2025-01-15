use bevy::prelude::*;
use bevy_http_client::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IpInfo {
    pub ip: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoginData {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub fn test_send_request(mut ev_request: EventWriter<TypedRequest<IpInfo>>) {
    ev_request.send(
        HttpClient::new()
            .get("https://api.ipify.org?format=json")
            .with_type::<IpInfo>(),
    );
}

pub fn test_handle_response(mut ev_response: EventReader<TypedResponse<IpInfo>>) {
    for response in ev_response.read() {
        println!("ip: {}", response.ip);
    }
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

pub fn handle_response_login(mut ev_response: EventReader<TypedResponse<LoginData>>) {
    for response in ev_response.read() {
        println!("ID: {}", response.id);
        println!("Token: {}", response.token);
    }
}

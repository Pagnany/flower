use bevy::prelude::*;
use bevy_http_client::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IpInfo {
    pub ip: String,
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

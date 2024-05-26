use crate::types::{
    connection::ServerConnection,
    token::{dummy_token_response, TokenResponse},
};
use reqwest::blocking::Client;
use std::collections::HashMap;

pub fn generate_token(username: &str, password: &str, connection: ServerConnection) -> String {
    let url = format!(
        "{}/v2/token/?ac={}",
        connection.url.clone(),
        connection.authcode.clone()
    );

    let mut params = HashMap::new();

    params.insert("username", username);
    params.insert("password", password);

    let call = Client::new().post(url.clone()).form(&params).send();

    match call {
        Ok(data) => {
            let text = data.text().unwrap_or_default();
            let json: TokenResponse = serde_json::from_str(&text).unwrap_or(dummy_token_response());

            return json.access_token;
        }
        Err(_) => dummy_token_response().access_token,
    }
}

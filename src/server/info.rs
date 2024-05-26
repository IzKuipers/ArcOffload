use crate::types::info::{dummy_server_info, dummy_server_info_str, ServerInfo};
use reqwest::blocking::{Client, Response};

pub fn get_server_info(server: String, authcode: String, is_https: bool, port: u16) -> ServerInfo {
    let protocol = if is_https { "https://" } else { "http://" };
    let url = format!("{protocol}{server}:{port}/v2/?ac={authcode}");
    let call: Result<Response, reqwest::Error> = Client::new().get(url).send();

    match call {
        Ok(data) => {
            let text = data.text().unwrap_or(dummy_server_info_str());
            let json: ServerInfo = serde_json::from_str(&text).unwrap_or(dummy_server_info());

            return json;
        }
        Err(_) => dummy_server_info(),
    }
}

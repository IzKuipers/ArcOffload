use reqwest::blocking::{Client, Response};

pub fn make_server_call<T: for<'a> serde::de::Deserialize<'a>>(
    endpoint: String,
    server: String,
    authcode: String,
    is_https: bool,
    port: u16,
    dummy: T,
    bearer: String,
) -> T {
    let protocol = if is_https { "https://" } else { "http://" };
    let url = format!("{protocol}{server}:{port}{endpoint}?ac={authcode}");
    let call: Result<Response, reqwest::Error> = Client::new().get(url).bearer_auth(bearer).send();

    return match call {
        Ok(data) => {
            let response = data.text().unwrap_or_default();
            let json: T = serde_json::from_str(response.as_str()).unwrap_or(dummy);

            return json;
        }
        Err(_) => dummy,
    };
}

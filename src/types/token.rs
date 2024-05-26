use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

pub fn dummy_token_response() -> TokenResponse {
    return TokenResponse {
        access_token: String::from(""),
    };
}

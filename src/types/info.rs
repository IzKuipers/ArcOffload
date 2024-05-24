use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ServerInfo {
    pub protected: bool,
    pub revision: i8,
    pub name: String,
}

pub fn dummy_server_info() -> ServerInfo {
    return ServerInfo {
        protected: false,
        revision: -1,
        name: String::from("dummy"),
    };
}

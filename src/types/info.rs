use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
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

pub fn dummy_server_info_str() -> String {
    return serde_json::to_string(&dummy_server_info()).unwrap();
}

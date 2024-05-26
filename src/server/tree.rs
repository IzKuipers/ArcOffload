use crate::types::{
    api::ApiResponse,
    connection::ServerConnection,
    tree::{tree_response_dummy, TreeEntry},
};
use reqwest::blocking::Client;

pub fn get_user_tree(token: String, connection: ServerConnection) -> Vec<TreeEntry> {
    let url = format!(
        "{}/fs/tree?ac={}",
        connection.url.clone(),
        connection.authcode.clone()
    );
    let call = Client::new().get(url.clone()).bearer_auth(token).send();

    match call {
        Ok(data) => {
            let text = data.text().unwrap_or_default();
            let json: ApiResponse<TreeEntry> =
                serde_json::from_str(&text).unwrap_or(tree_response_dummy());

            return json.data;
        }
        Err(_) => tree_response_dummy().data,
    }
}

use reqwest::blocking::Client;

use crate::types::{
    api::ApiResponse,
    tree::{tree_response_dummy, TreeEntry},
};

pub fn get_user_tree(
    token: String,
    server: String,
    authcode: String,
    is_https: bool,
    port: u16,
) -> Vec<TreeEntry> {
    let protocol = if is_https { "https://" } else { "http://" };
    let url = format!("{protocol}{server}:{port}/fs/tree?ac={authcode}");
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

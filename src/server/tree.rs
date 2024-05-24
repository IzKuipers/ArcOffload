use crate::{
    server::call::make_server_call,
    types::tree::{tree_response_dummy, TreeEntry},
};

pub fn get_user_tree(
    token: String,
    server: String,
    authcode: String,
    is_https: bool,
    port: u16,
) -> Vec<TreeEntry> {
    let call = make_server_call(
        "/fs/tree".to_string(),
        server,
        authcode,
        is_https,
        port,
        tree_response_dummy(),
    );
}

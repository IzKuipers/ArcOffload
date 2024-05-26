#[derive(Clone)]
pub struct ServerConnection {
    pub url: String,
    pub authcode: String,
}

pub fn server_connection(
    server: String,
    authcode: String,
    is_https: bool,
    port: u16,
) -> ServerConnection {
    let protocol = if is_https { "https://" } else { "http://" };
    let url = format!("{protocol}{server}:{port}");

    return ServerConnection { url, authcode };
}

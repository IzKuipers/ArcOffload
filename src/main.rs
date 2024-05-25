use ui::error::error_and_exit;

use crate::server::connect::connect_or_exit;

mod args;
mod server;
mod types;
mod ui;

fn main() {
    let arguments = args::parse_args();

    let server = arguments.server;
    let authcode = arguments.authcode;
    let is_https = arguments.is_https;
    let port = if is_https { 443 } else { arguments.port };

    let connected = connect_or_exit(server.clone(), authcode.clone(), is_https, port);

    if !connected {
        error_and_exit("You shouldn't be able to see this", 1);
    }

    let tree = server::tree::get_user_tree(
        String::from("8022984c-c0f3-4ff4-a7dc-cae29f234170"),
        server.clone(),
        authcode.clone(),
        is_https,
        port,
    );

    println!("{:#?}", tree);

    println!("blah")
}

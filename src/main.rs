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

    let connected = connect_or_exit(server, authcode, is_https, port);

    if !connected {
        error_and_exit("You shouldn't be able to see this", 1);
    }

    println!("blah")
}

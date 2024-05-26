use crate::{server::info::get_server_info, ui::error::error_and_exit};
use colored::Colorize;

pub fn connect_or_exit(server: String, authcode: String, is_https: bool, port: u16) -> bool {
    println!(
        "Calling up {} on port {} (presumably {})...",
        server.clone().blue().bold(),
        port.to_string().purple(),
        (if authcode.len() > 0 {
            "Protected"
        } else {
            "Public"
        })
        .yellow()
    );

    let call = get_server_info(server.clone(), authcode, is_https, port);

    if call.revision < 0 {
        error_and_exit(
            &format!("{} didn't pick up! Did you give me the right info?", server),
            1,
        );
    }

    return true;
}

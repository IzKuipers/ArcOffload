use crate::{
    types::info::{dummy_server_info, ServerInfo},
    ui::error::error_and_exit,
};
use colored::Colorize;

pub fn connect_or_exit(server: String, authcode: String, is_https: bool, port: u16) -> bool {
    println!(
        "\nCalling up {} on port {} ({})...\n",
        server.clone().blue().bold(),
        port.to_string().purple(),
        (if authcode.clone().len() > 0 {
            "Protected"
        } else {
            "Public"
        })
        .yellow()
    );

    let call = crate::server::call::make_server_call::<ServerInfo>(
        "/v2/".to_string(),
        server.clone(),
        authcode.clone(),
        is_https,
        port,
        dummy_server_info(),
    );

    if call.revision < 0 {
        error_and_exit(
            &format!(
                "{} didn't pick up! Did you give me the right info?",
                server.clone()
            ),
            1,
        );
    }

    return true;
}

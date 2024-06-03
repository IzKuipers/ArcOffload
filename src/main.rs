use crate::{
    server::{connect::connect_or_exit, download::write_full_tree, token::generate_token},
    ui::error::error_and_exit,
    util::read_str,
};
use colored::Colorize;
use dotenv;
use types::{
    connection::{server_connection, ServerConnection},
    tree::TreeEntry,
};
use ui::{ansi::try_ansi_or_exit, error::confirm_or_exit, intro::display_intro};

mod args;
mod env;
mod server;
mod types;
mod ui;
mod util;

fn main() {
    // =============================================================
    dotenv::dotenv().ok();
    let arguments = args::parse_args();

    let server = arguments.server;
    let authcode = arguments.authcode;
    let is_https = arguments.is_https;
    let port = if is_https { 443 } else { arguments.port };
    let dest = arguments.destination;
    let use_env = arguments.cred_from_env;
    // =============================================================

    try_ansi_or_exit();
    display_intro();

    let connection = server_connection(server.clone(), authcode.clone(), is_https, port);

    connect_or_exit(server, authcode, is_https, port);

    let token = stage_auth(connection.clone(), use_env);
    let tree = stage_get_tree(token.clone(), connection.clone());

    stage_go(tree, dest, token, connection, use_env);
}

fn stage_auth_env(connection: ServerConnection) -> String {
    let username = dotenv::var("OFFUSR").unwrap();
    let password = dotenv::var("OFFPWD").unwrap();

    let token = generate_token(&username, &password, connection);

    if token.len() == 0 {
        error_and_exit("Failed to authenticate!", 1);
    }

    return token;
}

fn stage_auth(connection: ServerConnection, use_env: bool) -> String {
    if use_env {
        return stage_auth_env(connection);
    }

    let username = read_str(&format!("\n{}", "Username".purple()));
    let password = rpassword::prompt_password(&format!("{}: ", "Password".purple())).unwrap();

    let token = generate_token(&username, &password, connection);

    if token.len() == 0 {
        error_and_exit("Failed to authenticate!", 1);
    }

    return token;
}

fn stage_get_tree(token: String, connection: ServerConnection) -> Vec<TreeEntry> {
    let tree = server::tree::get_user_tree(token, connection);

    if tree.len() == 0 {
        error_and_exit("Failed to get file list, do you even have files?", 1);
    }

    return tree;
}

fn stage_go(
    tree: Vec<TreeEntry>,
    dest: String,
    token: String,
    connection: ServerConnection,
    use_env: bool,
) {
    if !use_env {
        confirm_or_exit(&format!(
            "I will now start writing {} file(s) to {}.",
            tree.len().to_string().blue().bold(),
            dest.yellow().bold(),
        ));
    } else {
        println!(
            "\n{}: using {} for the user credentials.",
            "Note".yellow().bold(),
            ".env".blue()
        )
    }

    println!("");

    write_full_tree(tree, token, connection, dest);
}

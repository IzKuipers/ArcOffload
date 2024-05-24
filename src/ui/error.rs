use colored::Colorize;
use std::process::exit;

pub fn error_and_exit(message: &str, exit_code: i32) {
    println!("{}: {}", "Error".bright_red().bold(), message);
    exit(exit_code);
}

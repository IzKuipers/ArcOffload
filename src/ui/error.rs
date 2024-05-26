use colored::Colorize;
use std::process::exit;

use crate::util::read_str;

pub fn error_and_exit(message: &str, exit_code: i32) {
    println!("\n{}: {}", "Error".bright_red().bold(), message);
    exit(exit_code);
}

pub fn confirm_or_exit(prompt: &str) {
    let input = read_str(&format!("\n{} Confirm? [y/n]", prompt));

    if input.to_lowercase() != "y" {
        println!("Abort.");

        exit(0);
    }
}

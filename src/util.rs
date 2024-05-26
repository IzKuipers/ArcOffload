use std::io::{self, Write};

pub fn read_str(prefix: &str) -> String {
    print!("{}: ", prefix);
    io::stdout().flush().unwrap();
    let line = io::stdin().lines().next().unwrap().unwrap();

    return line;
}

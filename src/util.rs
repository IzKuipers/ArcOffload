use std::io;

pub fn read_str(prefix: &str) -> String {
    println!("{}: ", prefix);
    let line = io::stdin().lines().next().unwrap().unwrap();

    return line;
}

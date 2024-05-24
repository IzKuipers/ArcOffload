use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "The ArcAPI to connect to")]
    pub server: String,
    #[arg(
        short,
        long,
        default_value = "",
        help = "The Authcode, if any (used for protected servers)"
    )]
    pub authcode: String,
    #[arg(
        short,
        long,
        default_value_t = 3333,
        help = "The TCP port of the server"
    )]
    pub port: u16,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Specify if you want to connect using HTTPS"
    )]
    pub is_https: bool,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    return args;
}

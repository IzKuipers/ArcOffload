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
    #[arg(
        short,
        long,
        default_value = "out",
        help = "The directory to download the files to"
    )]
    pub destination: String,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Read the username and password from .env"
    )]
    pub cred_from_env: bool,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    return args;
}

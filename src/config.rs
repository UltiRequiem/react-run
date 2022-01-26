use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "UltiRequiem",
    version = "0.1.0",
    about = "Easily run React snippets"
)]
pub struct Args {
    pub file: String,
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    /// If passed your component will not be open on the browser automatically
    #[clap(short, long)]
    pub nobrowser: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}

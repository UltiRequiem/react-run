use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "UltiRequiem",
    version = "0.1.0",
    about = "Easily run React snippets"
)]
pub struct Args {
    #[clap(short, long)]
    pub file: String,
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
}

pub fn get_config() -> Args {
    Args::parse()
}

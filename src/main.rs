use clap::StructOpt;

pub mod build;
pub mod config;
pub mod server;

pub fn main() {
    let args = config::Args::parse();
    let app = build::build_react_app(&args.file);
    let port = format!("localhost:{}", args.port);

    server::serve(&app, &port, !args.simple);
}

pub mod build;
pub mod config;
pub mod server;

#[tokio::main]
async fn main() {
    let args = config::get_args();
    let app = build::react_app(&args.file);
    let port = format!("localhost:{}", args.port);

    server::serve(&app, &port, !args.nobrowser);
}

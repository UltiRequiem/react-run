pub mod build;
pub mod config;
pub mod server;

#[tokio::main]
pub async fn main() {
    let args = config::get_args();
    let app = build::react_app(&args.file).await;
    let port = format!("localhost:{}", args.port);

    server::serve(&app, &port, args.simple).await;
}

use clap::Parser;
use std::{fs, io::prelude::*, net};

static TEMPLATE: &'static str = include_str!("index.html");

#[derive(Parser, Debug)]
#[clap(
    author = "UltiRequiem",
    version = "0.1.0",
    about = "Easily run React snippets"
)]
struct Args {
    #[clap(short, long)]
    file: String,
    #[clap(short, long, default_value_t = 8080)]
    port: u16,
}

fn react_app(file: String) -> String {
    let app = fs::read_to_string(file).unwrap();
    TEMPLATE.replace("// APP", &app)
}

fn main() {
    let args = Args::parse();
    let app = react_app(args.file);
    let port = format!("127.0.0.1:{}", args.port);
    let listener = net::TcpListener::bind(&port).expect("Could not bind to port");

    println!("Listening on {}", &port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &app);
        print!("Ping!");
    }
}

fn handle_connection(mut stream: net::TcpStream, app: &String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        app.len(),
        app
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

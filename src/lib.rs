pub mod config;
use colored::Colorize;
use config::get_config;
use std::{fs, io::prelude::*, net, process};

static TEMPLATE: &str = include_str!("template.html");

pub fn run() {
    let args = get_config();
    let app = react_app(&args.file);
    let port = format!("localhost:{}", args.port);
    let listener = net::TcpListener::bind(&port).expect("Could not bind to port");

    println!("{}{}", "Listening on http://".blue(), &port.blue());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &app);
    }
}

fn react_app(file: &str) -> String {
    let app = match fs::read_to_string(file) {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Could not read file \"{}\": {}.", file.green(), e);
            process::exit(1);
        }
    };

    TEMPLATE.replace("// APP", &app)
}

fn handle_connection(mut stream: net::TcpStream, app: &str) {
    println!("[{}]", "Ping!".green());

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

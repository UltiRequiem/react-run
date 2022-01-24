pub mod config;
use clap::StructOpt;
use colored::Colorize;
use config::Args;
use open::that;
use std::{fs, io::prelude::*, net, process};

static TEMPLATE: &str = include_str!("template.html");

pub fn run() {
    let args = Args::parse();
    let app = react_app(&args.file);
    let port = format!("localhost:{}", args.port);

    let listener = match net::TcpListener::bind(&port) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!(
                "Cannot bind to port {}, probably is busy by other process: {}",
                &port, e
            );
            process::exit(1);
        }
    };

    println!("{}{}", "Listening on http://".blue(), &port.blue());

    that(format!("http://{}", &port)).unwrap();

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
    match stream.read(&mut [0; 1024]) {
        Ok(_) => println!("{}", "Request received.".green()),
        Err(error) => {
            eprintln!("Error reading the stream: {}", error);
            process::exit(1);
        }
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        app.len(),
        app
    );

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("[{}]", "Ping!".green()),
        Err(error) => {
            eprintln!("Could not write to stream: {}.", error);
            process::exit(1);
        }
    };

    stream.flush().unwrap();
}

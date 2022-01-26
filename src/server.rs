use colored::Colorize;
use open::that;
use std::{io::prelude::*, net, process};

// TODO: Manage Request asynchronously

pub async fn serve(app: &str, port: &str, open_on_browser: bool) {
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

    if open_on_browser {
        that(format!("http://{}", &port)).unwrap();
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &app);
    }
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

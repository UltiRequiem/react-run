use colored::Colorize;
use open::that;
use std::{io::prelude::*, net};

pub async fn serve(app: &str, addr: &str, nobrowser: bool) {
    let listener = net::TcpListener::bind(&addr)
        .unwrap_or_else(|e| panic!("Adders \"{}\" is already used: {}", addr, e));

    let url = format!("http://{}", addr);

    println!("{}{}", "Listening on ".blue(), url.cyan());

    if nobrowser {
        that(url).unwrap_or_else(|e| eprintln!("Failed to open your default browser: {}", e));
    };

    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), app);
    }
}

fn handle_connection(mut stream: net::TcpStream, app: &str) {
    let _bytes_proceeded = stream
        .read(&mut [0; 1024])
        .unwrap_or_else(|e| panic!("Failed to read from stream: {}", e));

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        app.len(),
        app
    );

    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|e| panic!("Could not write to stream: {}.", e));

    stream.flush().unwrap();

    println!("[{}]", "Ping!".green())
}

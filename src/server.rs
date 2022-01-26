use colored::Colorize;
use open::that;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

pub fn serve(app: &str, addr: &str, nobrowser: bool) {
    let server = TcpListener::bind(&addr)
        .unwrap_or_else(|e| panic!("Port {} is already used: {}", &addr, e));

    let url = format!("http://{}", &addr);

    println!("{}{}", "Listening on ".blue(), url.blue());

    if nobrowser {
        that(url).unwrap_or_else(|e| {
            eprintln!(
                "Failed to open the component on your default browser: {}",
                e
            );
        });
    };

    for stream in server.incoming() {
        handle_connection(stream.unwrap(), &app);
    }
}

fn handle_connection(mut stream: TcpStream, app: &str) {
    let mut buffer = [0; 1024];

    stream
        .read(&mut buffer)
        .unwrap_or_else(|e| panic!("Error reading the stream: {}", e));

    let root_route = b"GET / HTTP/1.1\r\n";
    let ws_route = b"GET /ws HTTP/1.1\r\n";

    if buffer.starts_with(root_route) {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            app.len(),
            app
        );

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("[{}]", "Ping!".green()),
            Err(error) => panic!("Could not write to stream: {}.", error),
        };
    } else if buffer.starts_with(ws_route) {
        let callback = |_req: &Request, mut response: Response| {
            let headers = response.headers_mut();
            headers.append("data", "reload".parse().unwrap());
            Ok(response)
        };
        let mut ws = accept_hdr(&stream, callback)
            .unwrap_or_else(|e| panic!("Error accepting handshake: {:?}", e));
        loop {
            let msg = ws.read_message().unwrap();
            if msg.is_binary() || msg.is_text() {
                ws.write_message(msg).unwrap();
            }
        }
    }

    stream.flush().unwrap();
}

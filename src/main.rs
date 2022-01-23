use std::{fs, io::prelude::*, net};

static TEMPLATE: &'static str = include_str!("index.html");

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:3000").expect("Could not bind to port");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let app = fs::read_to_string("examples/simple.js").unwrap();

    let contents = TEMPLATE.replace("// APP", &app);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

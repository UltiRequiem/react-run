use colored::Colorize;
use open::that;
use std::{io::prelude::*, net};

pub fn serve(app: &str, addr: &str, nobrowser: bool) {
    let listener = net::TcpListener::bind(&addr)
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

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &app);
    }
}

fn handle_connection(mut stream: net::TcpStream, app: &str) {
    match stream.read(&mut [0; 1024]) {
        Ok(_) => println!("{}", "Request received.".green()),
        Err(error) => panic!("Error reading the stream: {}", error),
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        app.len(),
        app
    );

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("[{}]", "Ping!".green()),
        Err(error) => panic!("Could not write to stream: {}.", error),
    };

    stream.flush().unwrap();
}

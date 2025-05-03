use hello::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// Starts the HTTP server and handles incoming connections using a thread pool.
///
/// Binds to `127.0.0.1:7878` and serves either the contents of `hello.html`,
/// or a 404 page (`404.html`) depending on the request path.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Accept a maximum of 2 incoming requests before shutting down
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Submit the request handling to the thread pool
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

/// Handles a single TCP connection by reading the HTTP request and writing an appropriate response.
///
/// - Responds with `hello.html` for `/` and `/sleep` routes.
/// - Introduces a 5-second delay for `/sleep`.
/// - Responds with `404.html` for all other routes.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}

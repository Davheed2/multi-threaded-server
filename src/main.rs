use std::{
    fs,
    io::{ prelude::*, BufReader },
    net::{ TcpListener, TcpStream },
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        eprintln!("Failed to bind TCP listener: {}", err);
        eprintln!("Make sure port 7878 is available and not in use");
        std::process::exit(1);
    });

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|err| {
            eprintln!("Failed to establish connection: {}", err);
            eprintln!("Connection attempt will be skipped");
            std::process::exit(1);
        });

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    //OR

    // for stream_result in listener.incoming() {
    //     match stream_result {
    //         Ok(stream) => {
    //             pool.execute(|| {
    //                 handle_connection(stream);
    //             });
    //         }
    //         Err(err) => {
    //             eprintln!("Error accepting connection: {}", err);
    //             eprintln!("Moving to next connection attempt");
    //             continue;
    //         }
    //     }
    // }

    //TESTING THREAD POOL WITH 2 CONNECTIONS
    // for stream in listener.incoming().take(2) {
    //     let stream = stream.unwrap_or_else(|err| {
    //         eprintln!("Failed to establish connection: {}", err);
    //         eprintln!("Connection attempt will be skipped");
    //         std::process::exit(1);
    //     });

    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }

    //OR

    // for stream_result in listener.incoming().take(2) {
    //     match stream_result {
    //         Ok(stream) => {
    //             pool.execute(|| {
    //                 handle_connection(stream);
    //             });
    //         }
    //         Err(err) => {
    //             eprintln!("Error accepting connection: {}", err);
    //             eprintln!("Moving to next connection attempt");
    //             continue;
    //         }
    //     }
    // }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    //println!("Request: {http_request:#?}");

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("index.html").unwrap();
    // let length = contents.len();

    // let response =
    //     format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();

    let request_line = buf_reader
        .lines()
        .next()
        .unwrap_or_else(|| {
            eprintln!("Failed to create empty string");
            std::process::exit(1);
        })
        .unwrap_or_else(|err| {
            eprintln!("Failed to read request line: {}", err);
            String::new()
        });

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "index.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Failed to read file '{}': {}", filename, err);
        eprintln!("Make sure the file exists and has correct permissions");
        String::from("Internal Server Error")
    });

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

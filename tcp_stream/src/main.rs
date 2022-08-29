// A TCP Stream testing tool in Rust
// By: @d4t4king

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stream = TcpStream::connect(format!("{}:{}", args[1], args[2])).expect("Connection failed");
    let mut buffer = [0; 1024];
    stream.write(args[3].as_bytes()).expect("Writing failed");
    thread::sleep(Duration::from_secs(1));
    stream.read(&mut buffer).expect("Reading failed");
    println!("Response: {}", String::from_utf8_lossy(&buffer[..]));
}

// To compile, run cargo build --release . This will create a binary in the target/release directory.
// 
// Now we need to create a Dockerfile to build the image.
// 
// # Dockerfile
// FROM rust:latest
// WORKDIR /usr/src/app
// COPY Cargo.toml .
// RUN mkdir src
// RUN echo "fn main() {}" > src/main.rs
// RUN cargo build --release
// COPY src src
// RUN rm ./target/release/deps/tcpstreamtest*
// RUN cargo build --release
// CMD ["./target/release/tcpstreamtest"]
// 
// To build the image, run docker build -t tcpstreamtest .
// 
// Now we can run the container with docker run tcpstreamtest

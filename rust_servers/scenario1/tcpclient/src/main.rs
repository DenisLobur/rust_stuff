use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    const hello_world: &str = "Hello, world!";
    stream.write(hello_world.as_bytes()).unwrap();
    let mut buffer = [0; hello_world.len()];
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}

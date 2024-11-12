use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 8096] = [0; 8096];
    //https://www.youtube.com/watch?v=JiuouCJQzSQ
}

fn main() {
    println!("Hello, world!");
}

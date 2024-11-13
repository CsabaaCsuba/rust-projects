use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub enum ParamDtype {
    int(i64),
    string(String),
}

pub enum method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

struct Request {
    method: method,
    endpoint: String,
    endpoint_params: Vec<ParamDtype>,

}

impl Request {
    fn new(raw_str: &str) -> Self {
        //TODO: this function should parse the raw request and construct the request struct which will be then used as parameters for the request's handlers
        todo!()
    }
}

struct Server {
    listener: TcpListener,


}

impl Server {
    fn new(address: &str) -> Self {
        Server {
            listener: TcpListener::bind(address).expect(&format!("failed to run server on address {address}"))
        }
    }
    fn get(&self, endpoint: &str, function: fn(Request) -> String) {

    }
}

fn handle_client(mut stream: TcpStream){
    let mut buffer: [u8; 8096] = [0; 8096];
    //https://www.youtube.com/watch?v=JiuouCJQzSQ

}

fn main() {
    println!("Hello, world!");
}

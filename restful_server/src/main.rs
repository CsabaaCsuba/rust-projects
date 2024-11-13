use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw;

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
    // endpoint_params: Vec<ParamDtype>, //not yet supported

    body: String,

}

impl Request {
    fn infer_method(used_str: &str) -> method{
        match &used_str[0..4] {
            "GET" => return method::GET,
            "PUT" => return method::PUT,
            _ => (),
        }
        if &used_str[0..5] == "POST" {
            return method::POST;
        }
        if &used_str[0..6] == "PATCH"{
            return method::PATCH;
        }
        if &used_str[0..7] == "DELETE"{
            return method::DELETE;
        }

        return method::GET; //if someone sends a SKIBIDI request, it will be interpreted as a GET request
    }
    fn infer_endpoint(used_str:&str) -> String {
        let mut split_string = used_str.split_whitespace();
        split_string.next();
        let potential_endpoint = split_string.next().clone();
        match potential_endpoint {
            Some(val) => return val.to_owned(),
            None => return "/".to_string() //if an endpoint is not specified, the index should be used
        }
    }
    fn infer_body(used_str: &str) -> String {
        let parts: Vec<&str> = used_str.splitn(2, "\r\n\r\n").collect();
        return parts[1].clone().to_owned();
    }


    fn new(raw_str: &str) -> Self {
        let mut i = 0;
        let method = Request::infer_method(&raw_str);
        let endpoint = Request::infer_endpoint(&raw_str);
        let body = Request::infer_body(&raw_str);
        Request {method, endpoint, body}

        //TODO: this function should parse the raw request and construct the request struct which will be then used as parameters for the request's handlers

    }
}

struct RequestHandler { //this will load the 
    used_method: method,
    endpoint_format: String, // /user/:id or /people-by-name/:string
    endpoint_params: Vec<ParamDtype>,
    handler_fn: fn(Request) -> String,
}

impl RequestHandler {
    fn parse_params(endpoint_str: &String) -> Vec<ParamDtype> {
        let mut params = Vec::new();
        let mut i: usize = 0;
        let iter_string = endpoint_str.clone();
        while i < endpoint_str.len(){
            if iter_string.get(i) == char::from_u32(b':' as u32){

            }

            i += 1;
        }

        params
    }
}

struct Server {
    address: String,
    listener: TcpListener,
    handlers: Vec<RequestHandler>,



}

impl Server {
    fn new(address: &str) -> Self {

        println!("setting up server at address {address}...");
        Server {
            listener: TcpListener::bind(address).expect(&format!("failed to run server on address {address}")),
            address: address.to_string(),
            handlers: Vec::new()
        }
        
    }
    fn handle_request(&self, mut stream: TcpStream) {
        let mut buffer: [u8; 8192] = [0; 8192];
        let read_result = stream.read(&mut buffer);
        if let Err(error) = read_result {
            eprintln!("Error happened while reading the buffer: {error:?}");
            return;
        }
        let request = String::from_utf8_lossy(&buffer);
        println!("received request: {}\n------------------", request);

    }
    fn run(&self) {
        println!("Server is running at address {}", &self.address);
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream)=> {
                    self.handle_request(stream);
                },
                Err(err)=> eprintln!("Error happened: {err:?}")
            }
        }
    }

    fn get(&self, endpoint: &str, function: fn(Request) -> String) {

    }

    fn post(&self, endpoint: &str, function: fn(Request) -> String) {

    }

    fn put(&self, endpoint: &str, function: fn(Request) -> String) {

    }
    fn patch(&self, endpoint: &str, function: fn(Request) -> String) {

    }
    fn delete(&self, endpoint: &str, function: fn(Request) -> String) {

    }
}



fn main() {
    let server = Server::new("127.0.0.1:8080");

    server.run();

}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use networking::GET;

#[derive(Debug, Clone, Copy)]
enum method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS
}

struct EndpointHandler {
    accepted_method: method,
    endpoint_format_str: String, //endpoints can be formatted like this: /books/{i32} -> i32 for the type
    handler_function: fn(String /*Takes in the request as a String and returns a JSON-based string (API response)*/) -> String,

}

struct Request {
    used_method: method,
    endpoint: String,
    header: String,
    body: String,
}
impl Request {
    fn from(whole_string: &str) -> Self {
        let m: method;
        if let Some(methodval) = check_method(whole_string) {
            m = methodval;
        }
        else {
            m = method::GET;
        }

        //content-length is always the last thing specified in a header
        let content_length_line = match whole_string.find("Content-Length:") {
            Some(idx) => &whole_string[idx..],
            None => panic!("content-length not found. Request doesn't adhere to HTTP standard")
        };
        let separator_idx = match separator.find("\n") { //the newline after Content-Length ensures 
            Some(idx) => idx,
            None => panic!("the request doesn't have an empty line"),
        };



        return Request {used_method:m, endpoint:String::from(""), body:String::from("")}
    }
}

struct Server {
    address: String,



}

impl Server {
    fn new(addr: &str) -> Self {
        return Server { address:addr.to_string()};
    }

    fn run(&self) {
        let listener = TcpListener::bind(self.address.clone()).expect("binding failed");

        println!("server is available on {}", self.address);

        for stream in listener.incoming(){
            match stream {
                Ok(stream) => {
                    std::thread::spawn(|| handle_client(stream));
                },
                Err(error) => eprintln!("Stream has an error: {:?}", error),
            }
        }
    }

    fn register(&self, methods: Vec<method>, endpoint: &str, function: fn(Request)->String) {

    }
}

// #[GET("/hi")]
// fn index() -> String{
//     String::from("{\"msg\":\"hi\"}")
// }



fn check_method(request_str: &str) -> Option<method>{
    if request_str[0..4].trim() == "GET"{
        return Some(method::GET);
    }
    if request_str[0..5].trim() == "POST"{
        return Some(method::POST);
    }
    if request_str[0..4].trim() == "PUT"{
        return Some(method::PUT);
    }
    if request_str[0..6].trim() == "PATCH"{
        return Some(method::PATCH);
    }
    if request_str[0..7].trim() == "DELETE"{
        return Some(method::DELETE);
    }
    if request_str[0..5].trim() == "HEAD"{
        return Some(method::HEAD);
    }
    if request_str[0..8].trim() == "OPTIONS"{
        return Some(method::OPTIONS);
    }
    else {
        return None;
    }
}

fn check_endpoint(request_str: &String) -> String{
    let mut return_string = String::with_capacity(64);
    let mut should_write = false;
    for char in request_str.chars(){
        if char == ' '{
            if !should_write {
                should_write = true;
                continue; //the first space isn't needed
            }
            else {
                break; //if it reaches the next space, break
            }
            
        }
        if should_write {
            return_string.push(char);
        }
    }


    return_string
}

fn send_response(stream: &mut TcpStream, response_code: u16, response_content: &str) {
    let status_line = format!("HTTP/1.1 {response_code}");
    let content_len = response_content.len();
    let response = format!("{status_line}\r\nContent-Length:{content_len}\r\n\r\n{response_content}");

    match &stream.write(response.as_bytes()) {
        Err(error) => eprintln!("Error happened during writing: {:?}", error),
        _ => (),
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Err(error) => eprintln!("Error happened: {:?}", error),
        _ => (),
    }


    let request = String::from_utf8_lossy(&buffer);
    // println!("found method: {:?}", );
    if let None = check_method(&request.to_string()) {
        eprintln!("invalid method specification at the start of the request");
        send_response(&mut stream, 400, "");
    }
    println!("found endpoint: {}", check_endpoint(&request.to_string()));
    println!("\n--------------------\nwhole request: \n{}\n--------------------\n", request);

    let response_content = String::from("{\"msg\":\"halo\"}");


    send_response(&mut stream, 200, &response_content);

    
}

fn main() {

    let server = Server::new("127.0.0.1:8080");
    
    server.run();
    
    
}

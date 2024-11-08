use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Server {
    endpoints: Vec<String>,
    

}

impl Server {
    fn new() -> Self {
        return Server { endpoints:vec![]};
    }
    
}

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

fn check_method(request_str: &String) -> Option<method>{
    if request_str[0..4].trim() == "GET"{
        println!("belemegy");
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

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Err(error) => eprintln!("Error happened: {:?}", error),
        _ => (),
    }


    let request = String::from_utf8_lossy(&buffer);
    println!("found method: {:?}", check_method(&request.to_string()));
    println!("found endpoint: {}", check_endpoint(&request.to_string()));
    println!("\n----------\nwhole request: \n{}----------\n", request);



    let status_line = "HTTP/1.1 200 OK";

    let response_content = "{\"msg\":\"hi from server\"}";
    let response_length = response_content.len();

    let response = format!("{status_line}\r\nContent-Length:{response_length}\r\n\r\n{response_content}");

    match stream.write(response.as_bytes()) {
        Err(error) => eprintln!("Error happened during writing: {:?}", error),
        _ => (),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("binding failed");
    println!("server is listening on 127.0.0.1:8080");
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            },
            Err(error) => eprintln!("Stream has an error: {:?}", error),
        }
    }
}

use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};

fn next_string_until_whitespace(input_str: &str) -> (&str, usize) {
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut found_normal = false;

    for char in input_str.chars() {
        if char == ' ' || char == '\t' || char == '\n' || char == '\r' {
            if found_normal {
                break;
            } else {
                start_idx += 1;
                end_idx += 1;
            }
        }
        else {
            if !found_normal {
                found_normal = true;
            }
            end_idx += 1;
        }
    }

    (&input_str[start_idx..end_idx], end_idx)
}

fn handle_client(mut stream: TcpStream){
    let mut buffer: [u8; 8192] = [0; 8192];
    //https://www.youtube.com/watch?v=JiuouCJQzSQ
    stream.read(&mut buffer).expect("failed to read");

    let request = String::from_utf8_lossy(&buffer[..]);

    let (method, endidx) = next_string_until_whitespace(&request);
    println!("parsed method: {method}");

    let (endpoint, endidx ) = next_string_until_whitespace(&request[endidx..]);
    println!("parsed endpoint: {endpoint}");

    println!("Remaining request: \n{}\n", &request[endidx..]);

    let response = "this is the answer to the request.";
    if let Err(error) = send_response(stream, response, 200){
        println!("Responding failed: {error}");
    };

}

fn send_response(mut stream: TcpStream, msg: &str, status_code: u16) -> Result<(), Error>{

    let content_length = msg.len();

    let response = format!("HTTP/1.1 {status_code}\r\nContent-Length:{content_length}\r\n\r\n{msg}");

    stream.write(response.as_bytes())?;

    Ok(())
}

fn main() {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).expect("failed to bind to address and port");
    println!("Server listening on {address}");
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            },
            Err(error) => {
                println!("Error happened: {error}");
            }
        }
    }
}

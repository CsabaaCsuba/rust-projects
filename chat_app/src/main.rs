//plans: User A --- User B's username ---> server --- User A wants to talk to you ---> Other User B
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Error};

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

pub struct User {
    username: String,
    password: String,
}

fn main() {
    let port = 8989;
    let ip = "127.0.0.1";
    let address = format!("{}:{}", ip, port);
    let mut users: Vec<User> = Vec::new();

    sign_in(&mut users, "Bro", "Kifli");
    sign_in(&mut users, "JohnPork", "Sonka");

    let listener = TcpListener::bind(&address).expect(&format!("Cannot bind to port {}", port));
    println!("Server running on {}", &address);
    for packet in listener.incoming() {
        if let Ok(stream) = packet {
            handle_connection(stream);
        }
        else {
            eprintln!("Error happened! Stream is not good")
        }

    }
}

pub fn sign_in(users: &mut Vec<User>, new_username: &str, new_password: &str){
    users.push(User {username: new_username.to_string(), password: new_password.to_string()});
}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), String>{
    let mut buf_reader = BufReader::new(&stream);

    let mut buffer: String = String::new();
    //data format: TEXT current_username reached_username
    let request_data = buf_reader.read_to_string(&mut buffer);
    if let Err(error) = request_data {
        return Err(format!("{:?}", error));
    }
    
    let data_vec: Vec<&str> = buffer.split(" ").collect();
    if data_vec.len() < 4 {
        return Err(String::from("Data is too short"));
    }
    if data_vec[0] != "TEXT" {
        return Err(format!("communication type {} unknown", data_vec[0]));
    }

    let current_user = data_vec[1];
    let wanted_user = data_vec[2];
    let message: Vec<&&str> = data_vec[3..].iter().collect();





    return Ok(())
}
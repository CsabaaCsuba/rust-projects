use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod own_connector;

// use orm::DBConnector;
mod orm;
use crate::orm::{Constraint, Type, Table, Field, DBConnector, DefaultValue};

mod json_parser;



#[derive(Debug, Clone, Copy)]
pub enum ParamDtype {
    int,
    string,
    float,
}

#[derive(Debug, Clone)]
pub enum ParamData {
    int(i64),
    string(String),
    float(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

#[derive(Debug, Clone, Copy)]
enum StatusCode {
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

impl StatusCode {
    // Function to get the numeric value of the status code
    fn value(&self) -> u16 {
        *self as u16
    }

    // Function to get the description of the status code
    fn description(&self) -> &'static str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NoContent => "No Content",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::NotModified => "Not Modified",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
        }
    }
}

struct Request {
    method: method,
    endpoint: String,
    params: Vec<ParamData>, //not yet supported

    body: String,
}

impl Request {
    fn infer_method(used_str: &str) -> method {
        let cmp_str = &used_str[0..6].to_uppercase();
        println!("unparsed method: {cmp_str}");
        match &cmp_str[0..3] {
            "GET" => return method::GET,
            "PUT" => return method::PUT,
            _ => (),
        }
        if &cmp_str[0..4] == "POST" {
            return method::POST;
        }
        if &cmp_str[0..5] == "PATCH" {
            return method::PATCH;
        }
        if &cmp_str[0..6] == "DELETE" {
            return method::DELETE;
        }

        println!("no method found. defaulting to GET.");
        return method::GET; //if someone sends a SKIBIDI request, it will be interpreted as a GET request
    }
    fn infer_endpoint(used_str: &str) -> String {
        let mut split_string = used_str.split_whitespace();
        split_string.next();
        let potential_endpoint = split_string.next().clone();
        match potential_endpoint {
            Some(val) => return val.to_owned(),
            None => return "/".to_string(), //if an endpoint is not specified, the index should be used
        }
    }
    fn infer_body(used_str: &str) -> String {
        let parts: Vec<&str> = used_str.splitn(2, "\r\n\r\n").collect();
        return parts[1].to_string();
    }

    fn new(raw_str: &str, params: Vec<ParamData>) -> Self {
        // let mut i = 0;
        let method = Request::infer_method(&raw_str);
        let endpoint = Request::infer_endpoint(&raw_str);
        let body = Request::infer_body(&raw_str);
        // let params: Vec<ParamDtype> = Vec::new();
        Request {
            method,
            endpoint,
            params,
            body,
        }

        //TODO: this function should parse the raw request and construct the request struct which will be then used as parameters for the request's handlers
    }
}

struct RequestHandler {
    //this will load the
    used_method: method,
    endpoint_format: String, // /user/:id or /people-by-name/:string
    endpoint_params: Vec<ParamDtype>,
    handler_fn: fn(Request) -> (String, StatusCode),
}

#[derive(Debug, Clone, Copy)]
pub enum ParamParseError {
    MissingTypeInfo,
    WrongFormat,
}

impl RequestHandler {
    fn new(
        method: method,
        endpoint: &str,
        handler_fn: fn(Request) -> (String, StatusCode),
    ) -> Result<Self, ParamParseError> {
        // println!("endpoint in new function: {endpoint}");
        let parsed_params = RequestHandler::parse_params(&endpoint)?;
        // println!("Parsed param types: {parsed_params:?}");
        return Ok(RequestHandler {
            used_method: method,
            endpoint_format: endpoint.to_string(),
            endpoint_params: parsed_params,
            handler_fn: handler_fn,
        });
    }
    fn parse_type(endpoint_str: &str, idx: usize) -> Result<(usize, ParamDtype), ParamParseError> {
        if &endpoint_str.to_lowercase()[idx..idx + 3] == "int" {
            return Ok((idx + 3, ParamDtype::int));
        } else if &endpoint_str.to_lowercase()[idx..idx + 5] == "float" {
            return Ok((idx + 5, ParamDtype::float));
        } else if &endpoint_str.to_lowercase()[idx..idx + 6] == "string" {
            return Ok((idx + 6, ParamDtype::string));
        } else {
            return Err(ParamParseError::MissingTypeInfo);
        }
    }
    fn parse_params(endpoint_str: &str) -> Result<Vec<ParamDtype>, ParamParseError> {
        let mut params = Vec::new();
        let mut i: usize = 0;
        let characters: Vec<char> = endpoint_str.chars().collect();
        // println!("characters casted as .chars: {characters:?}");
        // println!("{:?}", characters[1]);
        while i < characters.len() {
            // println!("Current char: {:?} at index {}", &characters[i], i);
            if characters.get(i) == Some(&':') {
                // println!("Found parameter type");
                i += 1;
                let return_tup = RequestHandler::parse_type(endpoint_str, i)?;
                // println!("parameter type is {:?}", &return_tup.1);
                i = return_tup.0;
                params.push(return_tup.1);

                let next_char = characters.get(i);
                // println!("Next char after param: {next_char:?}");
                if next_char != Some(&'/') && next_char != None {
                    //after parsing a parameter like :id, only a slash can come (or the end of the string)

                    eprintln!(
                        "endpoint string: {},\n Expected slash or end of string, got {:?}",
                        endpoint_str, next_char
                    );
                    return Err(ParamParseError::WrongFormat);
                }
            }

            i += 1;
        }

        Ok(params)
    }
    fn parse_param(used_str: &str, dtype: ParamDtype, idx: usize) -> Option<(ParamData, usize)> {
        let end_idx = used_str[idx..]
            .find('/')
            .unwrap_or_else(|| used_str.len() - idx)
            + idx;
        let substring = &used_str[idx..end_idx];

        // println!("needed parameter dtype: {dtype:?}");
        // println!("Current substring: {substring}");

        match dtype {
            ParamDtype::int => {
                let parse_result = substring.parse::<i64>();
                if let Ok(int_val) = parse_result {
                    return Some((ParamData::int(int_val), end_idx + 1));
                } else {
                    return None;
                }
            }
            ParamDtype::float => {
                let parse_result = substring.parse::<f64>();
                if let Ok(float_val) = parse_result {
                    return Some((ParamData::float(float_val), end_idx + 1));
                } else {
                    return None;
                }
            }
            ParamDtype::string => {
                // Return the substring as it is for string
                return Some((ParamData::string(substring.to_string()), end_idx + 1));
            }
        }
    }
    fn compare_endpoints(&self, request_endpoint: &str) -> Option<Vec<ParamData>> {
        let req_endpoint: Vec<char> = request_endpoint.chars().collect();
        let right_endpoint: Vec<char> = self.endpoint_format.chars().collect();
        let mut i = 0;
        let mut pidx = 0; //param idx
        let param_vec = self.endpoint_params.clone();
        let param_count = param_vec.len();
        let mut params = Vec::new();
        while i < req_endpoint.len() {
            if let Some(&char) = right_endpoint.get(i) {
                if char == ':' {
                    // println!("Encountered parameter notation");
                    if pidx == param_count {
                        //if there is a parameter, but the maximum amount of parameters has been reached
                        // eprintln!("Parameter count reached.");
                        return None;
                    }
                    let found_param =
                        RequestHandler::parse_param(request_endpoint, param_vec[pidx], i);
                    if let Some(p) = found_param {
                        // println!("Found parameters' data: {:?}", p.0);
                        params.push(p.0);
                        i = p.1;
                        pidx += 1;
                    } else {
                        eprintln!("Parsing parameter failed.");
                        return None; //parsing failed
                    }
                } else {
                    i += 1;
                }
            }
        }

        println!("compared parameters: {params:?}");
        Some(params)
    }

    fn compare_request(&self, req: &mut Request) -> bool {
        // println!("found {:?} - needed {:?}", req.method, self.used_method);
        if req.method.clone() != self.used_method {
            return false;
        }
        match self.compare_endpoints(&req.endpoint) {
            None => return false,
            Some(param_vector) => {
                req.params = param_vector;
                // println!("Request parameters: {:?}", req.params);
                return true;
            }
        }
    }
    fn execute(&self, req: Request, stream: &mut TcpStream) {
        let (returned_json, status_code) = (self.handler_fn)(req);
        let status_line = format!(
            "HTTP/1.1 {} {}",
            status_code.value(),
            status_code.description()
        );
        let content_len = returned_json.len();
        let header = format!(
            "{status_line}\r\nContent-Type: application/json\r\nContent-Length: {content_len}\r\n"
        );

        let response = format!("{header}\r\n{returned_json}");

        let res = stream.write(response.as_bytes());
        if let Err(e) = res {
            eprintln!("Error happened during sending response: {e}");
        }
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
            listener: TcpListener::bind(address)
                .expect(&format!("failed to run server on address {address}")),
            address: address.to_string(),
            handlers: Vec::new(),
        }
    }
    fn handle_request(&self, mut stream: TcpStream) {
        let mut buffer: [u8; 8192] = [0; 8192];
        let read_result = stream.read(&mut buffer);
        if let Err(error) = read_result {
            eprintln!("Error happened while reading the buffer: {error:?}");
            return;
        }
        let request_str = String::from_utf8_lossy(&buffer);
        // println!("received request_str:\n{}\n------------------", request_str);
        let mut request = Request::new(&request_str, vec![]);
        //TODO: write the functionality that matches the request with it's requesthandler (if there is one)
        for handler in &self.handlers {
            if handler.compare_request(&mut request) {
                handler.execute(request, &mut stream);
                break;
            }
        }
    }
    fn run(&self) {
        println!("Server is running at address {}", &self.address);
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_request(stream);
                }
                Err(err) => eprintln!("Error happened: {err:?}"),
            }
        }
    }

    fn get(&mut self, endpoint: &str, function: fn(Request) -> (String, StatusCode)) {
        //TODO: make this create a request handler and append it to the request handlers
        // println!("found endpoint: {endpoint}");
        let handler = RequestHandler::new(method::GET, endpoint, function);
        match handler {
            Ok(h) => self.handlers.push(h),
            Err(e) => eprintln!("Error: cannot add handler: {e:?}"),
        }
    }

    fn post(&mut self, endpoint: &str, function: fn(Request) -> (String, StatusCode)) {
        let handler = RequestHandler::new(method::POST, endpoint, function);
        match handler {
            Ok(h) => self.handlers.push(h),
            Err(e) => eprintln!("Error: cannot add handler: {e:?}"),
        }
    }

    fn put(&mut self, endpoint: &str, function: fn(Request) -> (String, StatusCode)) {
        let handler = RequestHandler::new(method::PUT, endpoint, function);
        match handler {
            Ok(h) => self.handlers.push(h),
            Err(e) => eprintln!("Error: cannot add handler: {e:?}"),
        }
    }
    fn patch(&mut self, endpoint: &str, function: fn(Request) -> (String, StatusCode)) {
        let handler = RequestHandler::new(method::PATCH, endpoint, function);
        match handler {
            Ok(h) => self.handlers.push(h),
            Err(e) => eprintln!("Error: cannot add handler: {e:?}"),
        }
    }
    fn delete(&mut self, endpoint: &str, function: fn(Request) -> (String, StatusCode)) {
        let handler = RequestHandler::new(method::DELETE, endpoint, function);
        match handler {
            Ok(h) => self.handlers.push(h),
            Err(e) => eprintln!("Error: cannot add handler: {e:?}"),
        }
    }
}

fn get_name(req: Request) -> (String, StatusCode) {
    let param1 = req.params[0].clone();
    let param: String;
    match param1 {
        ParamData::string(val) => param = val,
        _ => return (String::new(), StatusCode::BadRequest),
    }

    let resp = format!("{{\"username\":\"{param}\"}}");
    return (resp, StatusCode::OK);
}

fn post_user(req: Request) -> (String, StatusCode) {
    let param1 = req.params[0].clone();
    let param: f64;
    match param1 {
        ParamData::float(val) => param = val,
        _ => return (String::new(), StatusCode::BadRequest),
    }

    let resp = format!("{{\"float found\":\"{param}\"}}");
    return (resp, StatusCode::OK);
}

fn delete_something(req: Request) -> (String, StatusCode) {
    let p1 = req.params[0].clone();

    let param: i64;
    match p1 {
        ParamData::int(val) => param = val,
        _ => return (String::new(), StatusCode::BadRequest),
    }

    let resp = format!("{{\"deleted_thing\":\"{param}\"}}");
    return (resp, StatusCode::NoContent);
}

fn main() {
    
    let mut server = Server::new("127.0.0.1:8080");

    let mut db = DBConnector::new("root", "", "localhost", 3306, "mylittledb").expect("creating database failed");

    db.add_table(table!(
        User
        id INT PRIMARY_KEY AUTO_INCREMENT;
        is_premium BOOL DEFAULT(0);
        username VARCHAR(255) UNIQUE NOT_NULL;
        email VARCHAR(255) UNIQUE NOT_NULL;
        password CHAR(255);
    ));

    let _ = db.describe("User");

    server.get("/users/:string", get_name);

    server.delete("/users/:int", delete_something);

    server.post("/users/:float", post_user);

    server.run();

    
}

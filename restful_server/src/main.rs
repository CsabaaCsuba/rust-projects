#[allow(unused)]
#[allow(dead_code)]

mod lib;
use lib::*;

mod orm;

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

fn put_something(req: Request) -> (String, StatusCode) {
    let p1 = req.params[0].clone();

    let param: i64;
    match p1 {
        ParamData::int(val) => param = val,
        _ => return (String::new(), StatusCode::BadRequest),
    }

    let resp = format!("{{\"updated_item_id\":\"{param}\"}}");
    return (resp, StatusCode::NoContent);
}

fn patch_something(req: Request) -> (String, StatusCode) {
    let p1 = req.params[0].clone();

    let param: i64;
    match p1 {
        ParamData::int(val) => param = val,
        _ => return (String::new(), StatusCode::BadRequest),
    }

    let resp = format!("{{\"patched_item_id\":\"{param}\"}}");
    return (resp, StatusCode::NoContent);
}

fn main() {
    
    let mut server = lib::Server::new("127.0.0.1:8080");

    

    server.get("/users/:string", get_name);

    server.delete("/users/:int", delete_something);

    server.post("/users/:float", post_user);

    server.put("/items/:int", put_something);

    server.patch("/items/:int", patch_something);

    server.run();
}

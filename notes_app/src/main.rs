use restful_server::{Server, Request, StatusCode, DBConnector};

use mysql::*;
use mysql::prelude::*;


fn main() {
    let tasks_db = DBConnector::new("root", "", "localhost", 3306, "tasks")
        .expect("Connecting to the database failed.");


    fn get_task(req: Request) -> (String, StatusCode){
        let task_id = match req.params.get(0) {
            Some(id) => id,
            _ => return ("wrong param type".to_string(), StatusCode::BadRequest)
        };

        let task: Vec<String, String, bool> = tasks_db
            .connection.query(format!("SELECT title, description, is_completed FROM tasks WHERE id={id}").as_str());
            
        
        
    
    }
    
    fn get_tasks(req: Request) -> (String, StatusCode){
        
    }
    
    fn upload_task(req: Request) -> (String, StatusCode){
        
    }
    
    fn modify_task(req: Request) -> (String, StatusCode){
        
    }
    
    fn delete_task(req: Request) -> (String, StatusCode){
        
    }
    // println!("Hello, world!");
    let server = Server::new("127.0.0.1:8080");

    server.get("")
}

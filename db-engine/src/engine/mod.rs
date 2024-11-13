use std::thread;
use crate::command_parser::{parser::Value, tokens::TypeNotation};

struct Table {
    table_name: String,
    column_names: Vec<String>,
    column_dtypes: Vec<TypeNotation>,
    
    stored_values: Vec<Vec<Value>>,

}



struct Database {
    database_name: String,
    tables: Vec<Table>,
}

struct Engine {
    databases: Vec<Database>,

}
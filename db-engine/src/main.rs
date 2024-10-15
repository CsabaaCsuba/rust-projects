static POSSIBLE_DTYPES: [&str; 5] = ["INT", "ULONG", "DOUBLE", "VARCHAR", "BOOL"];

pub enum COLUMN_DTYPES {
    INT,
    ULONG,
    DOUBLE,
    VARCHAR,
    BOOL
}

struct Column {
    name: String,
    dtypes: Vec<COLUMN_DTYPES>
}

struct Table {
    name: String,
    columns: Vec<Column>,
}

struct Database {
    name: String,
    collation: String, //goal collation: utf8_hungarian_ci
    character_set: String, //defaults to utf8

}




impl Database {
    #[inline]
    fn new(input_name: String, input_collation: String) -> Self {
        Self { name: input_name, collation: input_collation, character_set: String::from("utf8") }
    }
 }

struct Engine {
    databases: Vec<Database>,
    selected_database: Option<String>, //name of the database
}

impl Engine {
    #[inline]
    fn new() -> Self{
        Self { databases: Vec::new(), selected_database: None }
    }

    fn create_db(&mut self, database_name: String, database_collation: String){
        
        self.databases.push(Database::new(database_name, database_collation));
    }

    fn select_db(&mut self, database_name: String){
        for db in &self.databases {
            if db.name == database_name {
                println!("database selected: {}", database_name);
                self.selected_database = Some(database_name.clone());
                break;
            }
        }
        println!("database named \"{database_name}\" wasn't found");
    }
}





fn main() {
    println!("Hello, world!");
}

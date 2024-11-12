
// static POSSIBLE_DTYPES: [&str; 5] = ["INT", "ULONG", "DOUBLE", "VARCHAR", "BOOL"];
#[allow(non_camel_case_types)]
pub enum COLUMN_DTYPE {
    INT,
    // ULONG,
    DOUBLE,
    // VARCHAR,
    TEXT,
    BOOL,
}

struct Column {
    name: String,
    dtype: COLUMN_DTYPE,
}

impl Column {
    fn new(column_name: String, column_dtype: COLUMN_DTYPE) -> Self {
        Self {
            name: column_name,
            dtype: column_dtype,
        }
    }
}

struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    fn new_empty(table_name: String) -> Self {
        Self {
            name: table_name,
            columns: Vec::new(),
        }
    }

    fn add_column(&mut self, column_input: Column) {
        self.columns.push(column_input);
    }
}

struct Database {
    name: String,
    collation: String,     //goal collation: utf8_hungarian_ci
    character_set: String, //defaults to utf8
    tables: Vec<Table>,
}

impl Database {
    #[inline]
    fn new(input_name: String, input_collation: String) -> Self {
        Self {
            name: input_name,
            collation: input_collation,
            character_set: String::from("utf8"),
            tables: Vec::new(),
        }
    }

    fn create_table(&mut self, table_name: &String){


    }

    fn drop_table(&mut self, table_name: &String) {
        let mut i = 0;
        let mut is_found = false;
        for table in &self.tables {
            if &table.name == table_name {
                is_found = true;
            }
            i += 1;
        }

        if is_found {
            let _ = &self.tables.remove(i);
            println!("table named \"{table_name}\" was removed");
        } else {
            println!("table named \"{table_name}\" was not found");
        }
    }
}

struct Engine {
    databases: Vec<Database>,
    selected_database: Option<String>, //name of the database
    selected_db_i: Option<usize>, //the index of the database in the db vector
}

impl Engine {
    #[inline]
    fn new() -> Self {
        Self {
            databases: Vec::new(),
            selected_database: None,
            selected_db_i: None,
        }
    }

    fn create_db(&mut self, database_name: String, database_collation: String) {
        self.databases
            .push(Database::new(database_name, database_collation));
    }

    fn select_db(&mut self, database_name: String) {
        let mut i = 0;
        for db in &self.databases {
            if db.name == database_name {
                println!("database selected: {}", database_name);
                self.selected_database = Some(database_name.clone());
                self.selected_db_i = Some(i);
                break;
            }
            i += 1;
        }
        println!("database named \"{database_name}\" wasn't found");
    }

    fn drop_db(&mut self, database_name: String) {
        let mut i = 0;
        let mut is_found = false;
        for db in &self.databases {
            if db.name == database_name {
                if self.selected_database == Some(database_name.clone()) {
                    self.selected_database = None;
                }
                is_found = true;
                break;
            }
            i += 1;
        }
        if is_found {
            let _ = &self.databases.remove(i);
            println!("database named \"{database_name}\" was removed");
        } else {
            println!("database named \"{database_name}\" wasn't found");
        }
    }

    fn create_table (&mut self, table_name: &String){
        if self.selected_database == None || self.selected_db_i == None{
            println!("select or create a database first");
            return;
        }
        let idx: usize = self.selected_db_i.unwrap(); //it was already checked if the db index is 0, so unwrap is safe here
        
        &self.databases[idx].create_table(table_name);

    }
}

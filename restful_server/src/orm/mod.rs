//todo: write an enum which will have a field "value", which can have a string (which will contain sql-formatted info like id int not null...)

//then write a macro called !table, which will generate a vector of these values

//example code:
/*
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
*/

#![allow(unused)]
#![allow(dead_code)]

mod table_macro;

use mysql::*;
use mysql::prelude::*;
use table_macro::table;



#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    TINYINT,
    SMALLINT,
    MEDIUMINT,
    INT,
    BIGINT,
    FLOAT,
    DOUBLE,
    DECIMAL(u8, u8),

    BOOL,

    DATE,
    DATETIME,
    TIMESTAMP,
    TIME,
    YEAR,

    CHAR(u8),
    VARCHAR(u16),

    TINYTEXT(u8),
    TEXT(u16),
    LONGTEXT(u32),

    BINARY,
    VARBINARY,

    ENUM(Vec<String>),
    SET(Vec<String>)
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::TINYINT => "TINYINT".to_string(),
            Type::SMALLINT => "SMALLINT".to_string(),
            Type::MEDIUMINT => "MEDIUMINT".to_string(),
            Type::INT => "INT".to_string(),
            Type::BIGINT => "BIGINT".to_string(),
            Type::FLOAT => "FLOAT".to_string(),
            Type::DOUBLE => "DOUBLE".to_string(),
            Type::DECIMAL(p, s) => format!("DECIMAL({}, {})", p, s),
            Type::BOOL => "BOOL".to_string(),
            Type::DATE => "DATE".to_string(),
            Type::DATETIME => "DATETIME".to_string(),
            Type::TIMESTAMP => "TIMESTAMP".to_string(),
            Type::TIME => "TIME".to_string(),
            Type::YEAR => "YEAR".to_string(),
            Type::CHAR(len) => format!("CHAR({})", len),
            Type::VARCHAR(len) => format!("VARCHAR({})", len),
            Type::TINYTEXT(len) => format!("TINYTEXT({})", len),
            Type::TEXT(len) => format!("TEXT({})", len),
            Type::LONGTEXT(len) => format!("LONGTEXT({})", len),
            Type::BINARY => "BINARY".to_string(),
            Type::VARBINARY => "VARBINARY".to_string(),
            Type::ENUM(values) => format!("ENUM({})", values.join(", ")),
            Type::SET(values) => format!("SET({})", values.join(", ")),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Constraint {
    NotNull,
    Unique,
    Default(DefaultValue),
    PrimaryKey,
    ForeignKey { table: String, field: String },
    AutoIncrement,
}

impl Constraint {
    pub fn to_string(&self) -> String {
        match self {
            Constraint::NotNull => "NOT NULL".to_string(),
            Constraint::Unique => "UNIQUE".to_string(),
            Constraint::Default(value) => format!("DEFAULT {}", value.to_string()),
            Constraint::PrimaryKey => "PRIMARY KEY".to_string(),
            Constraint::ForeignKey { table, field } => {
                format!("REFERENCES {}({})", table, field)
            },
            Constraint::AutoIncrement => "AUTO_INCREMENT".to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum DefaultValue {
    String(String),
    Int(i64),
    Float(f64),
}

impl DefaultValue {
    pub fn to_string(&self) -> String {
        match self {
            DefaultValue::String(val) => format!("'{}'", val),  // Strings are quoted
            DefaultValue::Int(val) => val.to_string(),
            DefaultValue::Float(val) => val.to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Field {
    pub field_name: String,
    pub field_type: Type,
    pub constraints: Vec<Constraint>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct DBConnector {
    pub tables: Vec<Table>,
    pub db_name: String,
    pub pool: Pool,
    pub connection: PooledConn,
}

impl DBConnector {
    pub fn new(db_manager: &str, username: &str, password: &str, address: &str, port: usize, db_name: &str) -> Result<Self, mysql::Error> {

        match db_manager {
            "mysql" => {

            }
            _ => {
                panic!("not implemented");
            }
        }
        
        
        let init_pool = Pool::new(format!("{db_manager}://{username}:{password}@{address}:{port}").as_str())?;
        let mut init_conn = init_pool.get_conn()?;

        init_conn.query_drop(format!("CREATE DATABASE IF NOT EXISTS {db_name}"))?;

        init_conn.query_drop(format!("USE {db_name}"))?;

    
        Ok(DBConnector{tables: vec![], db_name: db_name.to_string(), pool: init_pool, connection: init_conn})
    }
    pub fn describe(&mut self, table_name: &str) -> Result<Vec<Row>, mysql::Error> {

        // Query the `information_schema.COLUMNS` for the given table
        let result: Vec<Row> = self.connection.exec(
            "SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_KEY, COALESCE(COLUMN_DEFAULT, 'NULL') AS COLUMN_DEFAULT
            FROM information_schema.COLUMNS
            WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?",
            (&self.db_name, table_name),
        )?;

        Ok(result)
    }
    pub fn add_table(&mut self, table: Table){
        //todo: check if table exists
        let rows: &Vec<Row> = &self.describe(&table.name).expect("table check failed");
        //if nothing related to the table found, create table
        if rows.is_empty(){
            let mut query = format!("CREATE TABLE {} (", table.name);

            for (i, field) in table.fields.iter().enumerate() {
                query += &format!("{} {}", &field.field_name, &field.field_type.to_string());

                if !field.constraints.is_empty() {
                    let constraints: Vec<String> = field
                        .constraints
                        .iter()
                        .map(|constraint| constraint.to_string())
                        .collect();
                    query += &format!(" {}", constraints.join(" "));
                }

                if i < table.fields.len() - 1 {
                    query += ", ";
                }
            }
            query += ");";
            println!("generated query: {query}");

            let _ = self.connection.query_drop(&query);
        }
        else {
            //TODO: modify table
            todo!();
            
            
        }
    }
}

mod tests;

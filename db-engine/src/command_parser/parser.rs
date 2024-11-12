use core::error;
//this will convert token arrays to easily "digestible" data for the database engine
use std::{collections::HashMap, env::current_exe, mem::discriminant};


use super::tokens::{SWrap, Token};
use super::tokens::TypeNotation;

pub fn check_next_token(checked_token: Option<&Token>, check_against: Token) -> bool {
    //checks if a vector.get() element even exists, and then if it's the right token
    match checked_token {
        Some(token) => return discriminant(token) == discriminant(&check_against),
        None => return false,
    }
}

//for matching long token patterns, where variations aren't really accepted
fn match_rigid_pattern(
    main_vector: &Vec<Token>,
    other_vector: Vec<Token>,
    main_vector_start_idx: usize,
) -> bool {
    let mut i: usize = main_vector_start_idx;
    let mut y: usize = 0;
    while y < other_vector.len() {
        if discriminant(&main_vector[i]) != discriminant(&other_vector[y]) {
            println!("{:?} does not match {:?}", main_vector[i], other_vector[y]);
            return false;
        }

        i += 1;
        y += 1;
    }

    true
}

struct DBCreationData {
    db_name: String,
    collation: String,
    charset: String,
}

trait TokenConstruct {
    //the validity checking will happen here. Since it can fail, it has to be unwrapped upon calling
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self> where Self: Sized;
}

fn error_db_create(msg: &str) -> Option<DBCreationData> {
    eprintln!("Error creating database: {msg}");
    return None;
}

impl TokenConstruct for DBCreationData {
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self> {
        let statement: DBCreationData;
        let db_name: String;
        let collation: String;
        let charset: String;

        let token_pattern = vec![
            Token::CREATE,
            Token::DATABASE,
            Token::NAME_UNIT(SWrap::Filler),
            Token::CHARACTER,
            Token::SET,
            Token::STRING_VALUE(SWrap::Filler),
            Token::COLLATE,
            Token::STRING_VALUE(SWrap::Filler),
            Token::SEMICOLON,
        ];

        if match_rigid_pattern(&token_vector, token_pattern, 0) {
            match &token_vector[2] {
                Token::NAME_UNIT(SWrap::Data(db_n)) => db_name = db_n.clone(),
                _ => todo!() //this can't happen logically
            }
            match &token_vector[5] {
                Token::STRING_VALUE(SWrap::Data(coll)) => collation = coll.clone(),
                _ => todo!()
            }
            match &token_vector[7] {
                Token::STRING_VALUE(SWrap::Data(chars)) => charset = chars.clone(),
                _ => todo!()
            }
        }
        else {
            return error_db_create("Expected syntax: CREATE DATABASE database_name CHARACTER SET \"utf8\" COLLATE \"utf8_unicode_ci\";");
        }

        statement = DBCreationData {
            db_name,
            collation,
            charset,
        };

        return Some(statement);

    }
}

pub enum Value {
    BOOL(bool),
    
    BIGINT(i64),
    INT(i32),
    TINYINT(i16),
    
    CHAR(SWrap),
    VARCHAR(SWrap),

    DATE(SWrap),
    TIME(SWrap),
    DATETIME(SWrap),
    TIMESTAMP(SWrap),
    
}

pub enum FieldAttributes {
    NOT_NULL,
    UNIQUE,
    DEFAULT(Value),
    AUTO_INCREMENT,
    // CHECK,
}

struct ForeignKeyRelation {
    local_field: Box<str>, //&'static str is a mess, so it's easier with Boxes
    foreign_table: Box<str>,
    foreign_field: Box<str>,
}

struct TableCreationData {
    table_name: String,
    field_count: i32,
    fields: Vec<(String, TypeNotation)>, //using tuples eliminates the need for a field_types field
    attribute_table: HashMap<String, Vec<FieldAttributes>>,

    foreign_keys: Vec<ForeignKeyRelation>,

}

impl TokenConstruct for TableCreationData {
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self> {
        let return_struct = TableCreationData {
            table_name: String::new(),
            field_count: 0,
            fields: vec![],
            attribute_table: HashMap::new(),
            foreign_keys: vec![],
        };




        Some(return_struct)
    }
}

struct InsertData { //the length specifies how many elements will be used in the row
    table_name: String,
    rows_inserted: usize, //this is the amount of rows inserted into the database (this is the outer vector's length)
    row_length: usize, //amount of fields in a row (this is the inner vector's length [this has to be the same everywhere])
    fields_data: Vec<Vec<Value>>, 
}

impl TokenConstruct for InsertData {
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self> {
        //todo: 1. check for rigid pattern INSERT INTO table_name (
        //2. after the parentheses, count the amount of fields until the closing parenthesis,
        //so there will be a row_length which will be enforced when counting the amount of values in each insert row


    }
}
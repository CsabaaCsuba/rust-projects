//this will convert token arrays to easily "digestible" data for the database engine
use std::mem::discriminant;

use super::tokens::TypeNotation;
use super::tokens::{SWrap, Token};

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
    other_vector: &Vec<Token>,
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

fn match_rigid_pattern_fixed(
    main_vector: &Vec<Token>,
    other_vector: Vec<Token>,
    main_vector_start_idx: usize,
    max_iterations: usize,
) -> bool {
    let mut i: usize = main_vector_start_idx;
    let mut y: usize = 0;
    while y < other_vector.len() && y < max_iterations {
        if discriminant(&main_vector[i]) != discriminant(&other_vector[y]) {
            println!("{:?} does not match {:?}", main_vector[i], other_vector[y]);
            return false;
        }

        i += 1;
        y += 1;
    }

    true
}

pub struct DBCreationData {
    db_name: String,
    collation: String,
    charset: String,
}

pub trait TokenConstruct {
    //the validity checking will happen here. Since it can fail, it has to be unwrapped upon calling
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self>
    where
        Self: Sized;
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

        if match_rigid_pattern(&token_vector, &token_pattern, 0) {
            match &token_vector[2] {
                Token::NAME_UNIT(SWrap::Data(db_n)) => db_name = db_n.clone(),
                _ => todo!(), //this can't happen logically
            }
            match &token_vector[5] {
                Token::STRING_VALUE(SWrap::Data(coll)) => collation = coll.clone(),
                _ => todo!(),
            }
            match &token_vector[7] {
                Token::STRING_VALUE(SWrap::Data(chars)) => charset = chars.clone(),
                _ => todo!(),
            }
        } else {
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
    SMALLINT(i16),
    TINYINT(i8),

    FLOAT(f32),
    DOUBLE(f64),

    TEXT(SWrap),
    CHAR(SWrap, usize),
    VARCHAR(SWrap, usize),

    DATE(SWrap),
    TIME(SWrap),
    DATETIME(SWrap),
    TIMESTAMP(SWrap),
}

// pub enum CheckType {
//     SmallerThan(Value),
//     LargerThan(Value),
// }

pub enum OnDeleteAction {
    CASCADE,
    SET_NULL,
    SET_DEFAULT,
    RESTRICT,
    // NO_ACTION, //this is too close to restrict, however it depends on implementation
}

#[allow(non_camel_case_types)]
pub enum Constraint {
    PRIMARY_KEY,
    NOT_NULL,
    UNIQUE,
    DEFAULT(Value),
    AUTO_INCREMENT,
    REFERENCES(String, String, OnDeleteAction),
    // CHECK(CheckType),
}

struct ForeignKeyRelation {
    local_field: Box<str>, //&'static str is a mess, so it's easier with Boxes
    foreign_table: Box<str>,
    foreign_field: Box<str>,
}

pub struct TableCreationData {
    table_name: String,
    fields: Vec<(String, TypeNotation, Vec<Constraint>)>, //using tuples eliminates the need for a field_types field
}

// #[derive(Debug, Clone, Copy)]
// pub enum OptionX2<T, T2> {
//     Some(T),
//     Other(T2),
//     None,
// }

fn TableCreationError(msg: &str) -> Option<TableCreationData> {
    eprintln!("Error creating table: {msg}");
    return None;
}

fn parse_constraints(token_vector: &Vec<Token>, idx: usize) -> Option<(Vec<Constraint>, usize)> {
    //parses row until comma or closing parentheses
    let mut constraints = Vec::new();
    let mut i = idx;

    let primary_key_template = vec![Token::PRIMARY, Token::KEY];
    let not_null_template = vec![Token::NOT, Token::NULL];

    let references_template = vec![
        Token::REFERENCES,
        Token::NO_BACKTICK_NAME_UNIT(SWrap::Filler),
        Token::OPEN_PARENTHESIS,
        Token::NO_BACKTICK_NAME_UNIT(SWrap::Filler),
        Token::CLOSING_PARENTHESIS,
        Token::ON,
        Token::DELETE,
    ];

    // let set_default_pattern = vec![Token::SET]
    let current_token = token_vector.get(i);
    loop {
        if match_rigid_pattern(&token_vector, &primary_key_template, i) {
            i += 2;
            constraints.push(Constraint::PRIMARY_KEY);
        } else if match_rigid_pattern(&token_vector, &not_null_template, i) {
            i += 2;
            constraints.push(Constraint::NOT_NULL);
        } else if match_rigid_pattern(&token_vector, &references_template, i) {
            let table_name = match token_vector.get(i + 1) {
                Some(Token::NO_BACKTICK_NAME_UNIT(SWrap::Data(data))) => data.clone(),
                _ => return None,
            };

            let field_name = match token_vector.get(i + 3) {
                Some(Token::NO_BACKTICK_NAME_UNIT(SWrap::Data(data))) => data.clone(),
                _ => return None,
            };

            i += references_template.len();

            let on_delete_action = match token_vector.get(i) {
                Some(Token::SET) => {
                    i += 1;
                    match token_vector.get(i) {
                        Some(Token::NULL) => OnDeleteAction::SET_NULL,
                        Some(Token::DEFAULT) => OnDeleteAction::SET_DEFAULT,
                        t => {
                            eprintln!("Expected NULL or DEFAULT after set, got {t:?}");
                            return None;
                        }
                    }
                }
                Some(Token::CASCADE) => OnDeleteAction::CASCADE,
                Some(Token::RESTRICT) => OnDeleteAction::RESTRICT,
                _ => {
                    eprintln!(
                        "Expected valid ON DELETE action (SET NULL | DEFAULT, CASCADE, RESTRICT)"
                    );
                    return None;
                }
            };
            i += 1;
            constraints.push(Constraint::REFERENCES(
                table_name,
                field_name,
                on_delete_action,
            ));
        } else if check_next_token(current_token, Token::UNIQUE) {
            i += 1;
            constraints.push(Constraint::UNIQUE);
        } else if check_next_token(current_token, Token::AUTO_INCREMENT) {
            i += 1;
            constraints.push(Constraint::AUTO_INCREMENT);
        } else if check_next_token(current_token, Token::DEFAULT) {
            i += 1;
            match token_vector.get(i) {
                Some(Token::INT_VALUE(intval)) => {
                    constraints.push(Constraint::DEFAULT(Value::BIGINT(intval.clone())));
                }
                Some(Token::BOOL_VALUE(boolean)) => {
                    constraints.push(Constraint::DEFAULT(Value::BOOL(boolean.clone())));
                }
                Some(Token::STRING_VALUE(SWrap::Data(stringval))) => {
                    constraints.push(Constraint::DEFAULT(Value::TEXT(SWrap::Data(
                        stringval.clone(),
                    ))));
                }
                Some(Token::DOUBLE_VALUE(realval)) => {
                    constraints.push(Constraint::DEFAULT(Value::DOUBLE(realval.clone())));
                }
                _ => {
                    eprintln!("After Default, a value should have been written");
                    return None;
                }
            }
            i += 1;
        } else if check_next_token(current_token, Token::COMMA)
            || check_next_token(current_token, Token::CLOSING_PARENTHESIS)
        {
            break; //an actual closing character has been encountered
        } else {
            eprintln!(
                "Got unimplemented token while parsing contraints: {:?}",
                current_token
            );
            return None;
        }
    }

    Some((constraints, i))
}

impl TokenConstruct for TableCreationData {
    /*CREATE TABLE table_name (id int not null, name CHAR(255),  */
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self> {
        let table_name: String;
        let mut fields: Vec<(String, TypeNotation, Vec<Constraint>)> = Vec::new();

        match token_vector.get(2) {
            Some(Token::NAME_UNIT(SWrap::Data(data))) | Some(Token::NO_BACKTICK_NAME_UNIT(SWrap::Data(data))) => {
                table_name = data.clone();
            },
            _ => return TableCreationError("AFTER CREATE TABLE, a table name should come. Accepted syntax: table_name OR `table_name`")
        }

        if !check_next_token(token_vector.get(3), Token::OPEN_PARENTHESIS) {
            return TableCreationError("Expected Opening Parentheses \"(\" after table name");
        }

        let mut i: usize = 4;
        let tokens_len = token_vector.len();
        while i < tokens_len {
            //goes until a closing parenthesis or a CONSTRAINT keyword was found...
            let field_column: (String, TypeNotation, Vec<Constraint>);
            let field_name: String;
            let field_type: TypeNotation;
            let mut field_constraints: Vec<Constraint> = Vec::new();
            match &token_vector.get(i) {
                Some(Token::NO_BACKTICK_NAME_UNIT(SWrap::Data(data))) => {
                    i += 1;
                    field_name = data.clone();
                    match &token_vector.get(i) {
                        Some(Token::TYPE_KEYWORD(found_type)) => {
                            i += 1;
                            field_type = found_type.clone();
                            while i < tokens_len {}
                        }
                        other => {
                            return TableCreationError(&format!(
                                "Expected field name, found {other:?}"
                            ))
                        }
                    }
                }
                t => return TableCreationError(&format!("Expected field name, found {t:?}")),
            }
            field_constraints = match parse_constraints(&token_vector, i) {
                Some(tup) => {
                    i = tup.1;
                    tup.0
                }
                None => return TableCreationError(&format!("Couldn't parse contraint")),
            };
            field_column = (field_name, field_type, field_constraints);
            fields.push(field_column);
        }

        let return_struct = TableCreationData {
            table_name,
            fields,
        };

        Some(return_struct)
    }
}

struct InsertData {
    //the length specifies how many elements will be used in the row
    table_name: String,
    rows_inserted: usize, //this is the amount of rows inserted into the database (this is the outer vector's length)
    row_length: usize, //amount of fields in a row (this is the inner vector's length [this has to be the same everywhere])
    fields_data: Vec<Vec<Value>>,
}

// impl TokenConstruct for InsertData {
//     fn from_tokens(token_vector: Vec<Token>) -> Option<Self> {
//         //todo: 1. check for rigid pattern INSERT INTO table_name (
//         //2. after the parentheses, count the amount of fields until the closing parenthesis,
//         //so there will be a row_length which will be enforced when counting the amount of values in each insert row

//     }
// }

use crate::command_parser::{tokens::StringWrapper, Token};
use std::{env::current_exe, mem::discriminant};

use super::tokens::{IsSequenceValid};

fn error_create(msg: &str) -> bool {
    println!("create statement: validation error: {msg}");
    return false;
}

// fn match_token(vector: &Vec<Token>, idx: usize, matched_token: Token){ //would be too slow compared to just checking the token's length
//     match token
// }

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

pub fn validate_create(token_vector: &Vec<Token>) -> bool {
    //CREATE -> DATABASE -> `database_name` -> COLLATE -> "collation" (optional) -> CHARACTER -> SET -> "charset" (optional);
    //CREATE -> TABLE -> `table_name` -> (column_name TYPE [optional args], column_name TYPE [optional args]);
    println!("validating create statement");

    let tokens_len = token_vector.len();
    if tokens_len < 4 || token_vector[0] != Token::CREATE {
        println!(
            "Wrong first token or wrong command length: {:?}",
            token_vector
        );
        return false;
    }

    match &token_vector[2] {
        Token::NAME_UNIT(string) | Token::NO_BACKTICK_NAME_UNIT(string) => {
            println!("Name unit is {string:?}")
        }
        _ => return error_create("incorrect name for the database"),
    }
    let mut i = 3;

    if token_vector[1] == Token::DATABASE {
        while i < tokens_len {
            if token_vector[i] == Token::SEMICOLON {
                println!("semicolon encountered");
                return true; //if it's a semicolon, return true
            } else if token_vector[i] == Token::COLLATE {
                println!("collate keyword encountered");
                match token_vector.get(i + 1) {
                    Some(found_token) => match found_token {
                        Token::STRING_VALUE(string_val) => {
                            println!("Collation string value: {string_val:?}")
                        }
                        _ => return error_create("collation isn't specified properly"),
                    },
                    _ => return error_create("collation isn't specified"), //if the token is not a string
                }
                i += 2; //the next token is a complementer token for "COLLATE", which is checked, so no need to return
            } else if token_vector[i] == Token::CHARACTER {
                println!("character keyword encountered");
                match token_vector.get(i + 1) {
                    Some(found_token) => {
                        match found_token {
                            Token::SET => println!("\"SET\" keyword encountered"), //if the found token is a string token
                            _ => return error_create("CHARACTER keyword must be followed by SET"),
                        }
                    }
                    None => return error_create("nothing found after CHARACTER keyword"), //if the token is not a string
                }
                match token_vector.get(i + 2) {
                    Some(found_token) => {
                        match found_token {
                            Token::STRING_VALUE(string_val) => {
                                println!("String value: {string_val:?}")
                            } //if the found token is a string token
                            _ => return error_create("character set is not valid"),
                        }
                    }
                    _ => return error_create("character set was not specified"), //if the token is not a string
                }
                i += 3;
            } else {
                return error_create(
                    format!("unrelated keyword was used: {:?}", token_vector[i]).as_str(),
                ); //if the token after the unit
                   //naming token isn't a CHARACTER SET or a COLLATE, false should be returned;
            }
        }
    }
    //TODO: write a program which validates create table commands
    else if token_vector[1] == Token::TABLE {
        if token_vector[3] != Token::OPEN_PARENTHESIS {
            return error_create("type specification part is not found");
        }
        #[allow(non_camel_case_types)]
        #[derive(PartialEq)]
        enum Pattern {
            nothing,
            base,
            keydef, //for foreign / primary key definitions
        }
        let mut found_pattern = Pattern::nothing; //whether if the found_pattern was encountered or not
        let mut i = 4;
        while i < tokens_len {
            if token_vector[i] == Token::CLOSING_PARENTHESIS {
                //in this type of statements, the closing parenthesis also marks the end of the statement basically
                if check_next_token(token_vector.get(i + 1), Token::SEMICOLON) {
                    if found_pattern != Pattern::nothing {
                        //if colon was definied after a base pattern (because if it wasn't, an unnecessary comma was put into the command)
                        println!("semicolon encountered");
                        return true; //if it's a semicolon, return true
                    } else {
                        return error_create(
                            "unnecessary \",\" before the closing parenthetis \")\"",
                        );
                    }
                } else {
                    return error_create("statement isn't closed properly");
                }
            }

            match token_vector[i] {
                Token::FOREIGN => {
                    // FOREIGN KEY ( some_id_field ) REFERENCES other_table ( other_id_field ) ;
                    if found_pattern != Pattern::nothing {
                        return error_create("pattern found before foreign key, consider putting a comma before \"FOREIGN\"");
                    }
                    i += 1;
                    let foreign_syntax = vec![
                        Token::KEY,
                        Token::OPEN_PARENTHESIS,
                        Token::NO_BACKTICK_NAME_UNIT(StringWrapper::Filler),
                        Token::CLOSING_PARENTHESIS,
                        Token::REFERENCES,
                        Token::NO_BACKTICK_NAME_UNIT(StringWrapper::Filler),
                        Token::OPEN_PARENTHESIS,
                        Token::NO_BACKTICK_NAME_UNIT(StringWrapper::Filler),
                        Token::CLOSING_PARENTHESIS,
                        Token::ON,
                        Token::DELETE,
                    ];
                    let syntax_len = foreign_syntax.len(); //because it will get out of scope
                    if match_rigid_pattern(token_vector, foreign_syntax, i) {
                        i += syntax_len;
                        println!("rigid pattern was correct");
                    } else {
                        return error_create("FOREIGN KEY pattern invalid");
                    }
                    //TODO: check the things that can come after on-delete (CASCADE, SET NULL, SET DEFAULT, NO ACTION, RESTRICT)
                    match token_vector.get(i){
                        Some(token) => {
                            match token {
                                Token::CASCADE | Token::RESTRICT => i += 1,
                                Token::SET => {
                                    if check_next_token(token_vector.get(i + 2), Token::NULL){
                                        i += 2;
                                    }
                                    else if check_next_token(token_vector.get(i + 2), Token::DEFAULT){
                                        i += 2;
                                    }
                                    else {
                                        return error_create("after SET, NULL and DEFAULT can come");
                                    }
                                }
                                Token::NO => {
                                    if check_next_token(token_vector.get(i + 2), Token::ACTION){
                                        i += 2;
                                    }
                                    else {
                                        return error_create("after NO, ACTION can come in this context only");
                                    }
                                }
                                _ => return error_create(format!("{token:?}unexpected keyword after ON DELETE (the right ones: CASCADE, SET NULL, SET DEFAULT, NO ACTION, RESTRICT)").as_str())
                            }
                        }
                        None => return error_create("expected keyword after ON DELETE (CASCADE, SET NULL, SET DEFAULT, NO ACTION, RESTRICT)")
                    }

                    // if check_next_token(token_vector.get(i), Token::COMMA){
                    //     i += 1;
                    // }
                    // else {
                    //     return error_create("After foreign key definition, a comma should be put");
                    // }
                    found_pattern = Pattern::keydef; //this was a complete base pattern, a comma can come after it
                }

                Token::NO_BACKTICK_NAME_UNIT(_) => {
                    // if i+1 >= tokens_len { //easy checking implemented, so the code has less match statements
                    //     return error_create(""); //vector would be out of bounds
                    // }
                    match token_vector.get(i + 1) {
                        Some(found_token) => {
                            //THIS MIGHT NOT WORK
                            match found_token {
                                Token::TYPE_KEYWORD(_) => {
                                    println!("type keyword found");
                                    //if it's a type, then optional keywords can come like PRIMARY KEY or NOT NULL
                                    found_pattern = Pattern::base;
                                    i += 2;
                                }
                                _ => {
                                    return error_create(
                                        "wrong type keyword after field name specification",
                                    )
                                }
                            }
                        }
                        None => return error_create("Keyword not found after column name"),
                    }
                }
                //the comma resets the base pattern, because in the next loop, basic tokens can come as well
                Token::COMMA => {
                    if found_pattern != Pattern::nothing {
                        println!("column definition ended. next column...");
                        found_pattern = Pattern::nothing;
                        i += 1;
                    } else {
                        return error_create("comma at wrong space");
                    }
                }
                //todo: make this part of the code more concise
                Token::NOT => {
                    if found_pattern == Pattern::base {
                        println!("base pattern was already found. NOT keyword is legitimate");
                        if check_next_token(token_vector.get(i + 1), Token::NULL) {
                            println!("after NOT, NULL token is there");
                            i += 2; //if the next token is NULL, it's valid, but nothing has to be done apart from incrementing the loop
                        } else {
                            return error_create("NOT keyword must be followed by NULL");
                        }
                    } else {
                        return error_create("NOT NULL must be used after field name and type");
                    }
                }
                Token::PRIMARY => {
                    if check_next_token(token_vector.get(i + 1), Token::KEY) {
                        println!("after PRIMARY, KEY token is there");
                        i += 2; //if the next token is NULL, it's valid, but nothing has to be done apart from incrementing the loop
                        if found_pattern == Pattern::base {
                            println!("token marks only one primary key");
                        } else {
                            println!("token marks composite primary keys");
                            if !check_next_token(token_vector.get(i), Token::OPEN_PARENTHESIS) {
                                return error_create(
                                    "opening parenthesis must come after PRIMARY KEY",
                                );
                            }
                            i += 1;
                            while check_next_token(
                                token_vector.get(i),
                                Token::NO_BACKTICK_NAME_UNIT(StringWrapper::Filler),
                            ) {
                                i += 1;
                                if check_next_token(token_vector.get(i), Token::COMMA) {
                                    println!("comma found after primary key");
                                    i += 1;
                                } else if check_next_token(
                                    token_vector.get(i),
                                    Token::CLOSING_PARENTHESIS,
                                ) {
                                    // i += 1;
                                } else {
                                    return error_create(
                                        "Closing parenthesis missing after composite primary keys",
                                    );
                                }
                            }

                            i += 1;
                            found_pattern = Pattern::keydef; //so a comma can check it's end
                            continue;
                        }
                    } else {
                        return error_create("PRIMARY must be followed by KEY");
                    }
                }
                Token::AUTO_INCREMENT => {
                    if found_pattern == Pattern::base {
                        println!("auto increment token was found in it's rightful place");
                        i += 1;
                    } else {
                        return error_create("AUTO_INCREMENT was used incorrectly");
                    }
                }
                Token::UNIQUE => {
                    if found_pattern == Pattern::base {
                        println!("unique keyword was found in it's right place");
                        i += 1;
                    } else {
                        return error_create("UNIQUE should be after a keyword");
                    }
                }
                // Token::SEMICOLON => {
                //     println!("found semicolon");
                //     return true;
                // }
                //TODO: Add other optional keywords here
                _ => {
                    return error_create(
                        format!(
                            "wrong keyword in create table statement: {:?}",
                            token_vector[i]
                        )
                        .as_str(),
                    )
                }
            }
        }
    } else {
        return error_create("you can't create anything apart from a DATABASE and a TABLE");
    }

    println!("valid create statement.\n");
    true
}

fn check_and_or_relation(token_vector: &Vec<Token>, start_index: usize) -> IsSequenceValid {
    //checks for a condition, and whether if it's an and / or clause. Obviously returns false if it fails
    //if it encounters an opening parenthesis, recursion will happen
    let mut i = start_index;

    let tokens_len = token_vector.len();
    while i < tokens_len { //each iteration checks for a whole expression: num < 3 | string = "stringval" | string2 LIKE "%val%" 
        match token_vector.get(i){
            Some(Token::OPEN_PARENTHESIS) => {
                i += 1;
                match check_and_or_relation(token_vector, i) {
                    IsSequenceValid::True(val) => i = val, //increases i to val
                    IsSequenceValid::False => {println!("failed to parse where relation"); return IsSequenceValid::False},
                }
            }
            Some(Token::NO_BACKTICK_NAME_UNIT(_)) => {
                match token_vector.get(i+2) { //jump ahead and check if it's a number or text or bool
                    Some(Token::INT_VALUE(_)) | Some(Token::DOUBLE_VALUE(_)) => {
                        match token_vector.get(i+1) {
                            Some(Token::EQUAL_SIGN) | Some(Token::SMALLER_THAN_OPERATOR) | Some(Token::BIGGER_THAN_OPERATOR) => {println!("number comparison valid")}
                            _ => {println!("number comparison isn't valid, use <, >, or ="); return IsSequenceValid::False}
                        }
                    }
                    Some(Token::BOOL_VALUE(_)) => {
                        if check_next_token(token_vector.get(i+1), Token::EQUAL_SIGN) {
                            println!("boolean comparison legit")
                        }
                        else {
                            println!("boolean comparison didn't have an equal sign");
                            return IsSequenceValid::False;
                        }
                    }
                    Some(Token::STRING_VALUE(_)) => {
                        match token_vector.get(i+1) {
                            Some(Token::EQUAL_SIGN) | Some(Token::LIKE)=> {println!("string comparison valid")}
                            _ => {println!("string comparison isn't valid, use either = or \"LIKE\""); return IsSequenceValid::False}
                        }
                    }
                    None => {println!("where clause ended before it should have..."); return IsSequenceValid::False}
                    _ => {
                        println!("the other side of the comparison should have a value as well.");
                        return IsSequenceValid::False;
                    }
                }

                i += 3; //it can be incremented by 3, since the next 2 tokens were already checked
            }

            _ => {println!("Invalid token in where clause"); return IsSequenceValid::False}
        }
    }

    

    IsSequenceValid::True(i)
}

fn check_where_clause(token_vector: &Vec<Token>, start_index: usize) -> IsSequenceValid {
    //where can only have < or > between numbers, and LIKE is only for strings
    if !check_next_token(token_vector.get(start_index), Token::WHERE){
        return IsSequenceValid::False;
    }
    let mut i = start_index;
    while i < token_vector.len() {


        i += 1;
    }

    IsSequenceValid::True(i)
}

fn error_select(msg: &str) -> bool {
    println!("insert syntax error: {msg}");
    false
}

pub fn validate_select(token_vector: &Vec<Token>) -> bool {
    // SELECT -> COLUMNS | * -> FROM `table_name` -> WHERE (optional) -> AND / OR (if there's an other criteria)
    // -> GROUP BY (optional) -> HAVING (optional) -> ORDER BY (optional) -> LIMIT (optional);
    let tokens_len = token_vector.len();
    if tokens_len < 5{
        return error_select("there aren't even enough tokens");
    }
    if !check_next_token(token_vector.get(0), Token::SELECT){
        return error_select("how is it even possible to not start a select statement with select...");
    }
    //after select, the columns must come, and the columns end when there is no "," and there is a FROM
    let mut i = 1;
    let mut last_token: Token = Token::SELECT; //this is the first token anyway
    while i < tokens_len {
        let current_token = &token_vector[i];
        if check_next_token(Some(current_token), Token::NO_BACKTICK_NAME_UNIT(StringWrapper::Filler)){
            i += 1;
            match token_vector.get(i){
                Some(Token::COMMA) => {i += 1; last_token = current_token.clone()},
                Some(Token::FROM) => {
                    // i += 1; //this increment is not needed
                    break;
                },
                None => return error_select(format!("nothing found after column {current_token:?} to select...").as_str()),
                _ => return error_select(format!("Reserved keyword found after {current_token:?}").as_str()),
            }
        }
        else {
            return error_select(format!("wrong token of text after {last_token:?}").as_str());
        }
    }

    match token_vector.get(i+1){
        Some(Token::NO_BACKTICK_NAME_UNIT(_)) => {},
        Some(Token::NAME_UNIT(_)) => {},
        Some(token) => return error_create(format!("token {token:?} is wrong after {:?}", token_vector[i]).as_str()),
        None => return error_create("Nothing found after \"FROM\""),
    }
    i += 2;

    /*
    order:
    1. where
    2. group by
    3. having
    4. order by
    5. limit
     */

    let where_result: IsSequenceValid = check_where_clause(token_vector, i);
    match where_result {
        IsSequenceValid::True(val) => i = val,
        IsSequenceValid::False => return error_select("Error in WHERE clause"),
    }
    let mut found_statements = [false, false, false, false];
    while i < tokens_len {

    }




    
    true
}

fn error_insert(error_msg: &str) -> bool {
    println!("Error inserting: {error_msg}");
    return false;
}

pub fn validate_insert(token_vector: &Vec<Token>) -> bool {
    // INSERT INTO `table_name` -> (columns) (optional) -> VALUES (values_list) -> ON DUPLICATE KEY UPDATE (optional);
    //TODO: write the longer variant (column definitions as well)
    let tokens_len = token_vector.len();
    if tokens_len < 6 {
        return error_insert("insert statement too short");
    }
    //checking the starting - default cases
    if !check_next_token(token_vector.get(0), Token::INSERT) {
        return error_insert("the first token is not an insert token");
    }
    if !check_next_token(token_vector.get(1), Token::INTO) {
        return error_insert("the second token is not \"INTO\"");
    }
    match token_vector.get(2) {
        Some(token) => match token {
            Token::NO_BACKTICK_NAME_UNIT(_) => (),
            Token::NAME_UNIT(_) => (),
            _ => return error_insert("table name incorrect."),
        },
        None => return error_insert("sentence too short"),
    }
    // if !check_next_token(token_vector.get(3), Token::OPEN_PARENTHESIS) {
    //     return error_insert("opening parenthesis for the columns not found");
    // }
    match token_vector[3] {
        Token::VALUES => {
            //omitting column order definition
            println!("short syntax used");
            if !check_next_token(token_vector.get(4), Token::OPEN_PARENTHESIS) {
                println!("expected opening parenthesis after \"VALUES\" keyword");
                return false;
            }
            let mut i = 5;
            let mut values_count = 0;
            let mut current_rows_values = 0;
            let mut is_first_row = true;
            while i < tokens_len {
                match token_vector.get(i) {
                    Some(found_token) => match found_token {
                        Token::STRING_VALUE(_)
                        | Token::BOOL_VALUE(_)
                        | Token::INT_VALUE(_)
                        | Token::DOUBLE_VALUE(_) => {
                            println!("found value: {found_token:?}");
                            i += 1;
                            if is_first_row {
                                values_count += 1;
                            } else {
                                current_rows_values += 1;
                                if current_rows_values > values_count {
                                    return error_create(
                                        "there are incosistent data values in rows",
                                    );
                                }
                            }
                            match token_vector.get(i) {
                                Some(Token::COMMA) => {
                                    println!("comma found after value. expecting more values.");
                                    i += 1;
                                    continue;
                                }
                                Some(Token::CLOSING_PARENTHESIS) => {
                                    if !is_first_row {
                                        if current_rows_values != values_count {
                                            return error_create("last parsed data row's value count is different from the first one.");
                                        }
                                    }
                                    println!("closing parenthesis found. values closed.");
                                    i += 1;
                                    if check_next_token(token_vector.get(i), Token::SEMICOLON) {
                                        println!("semicolon found after opening parenthesis. Statement valid.");
                                        return true;
                                    } else if check_next_token(token_vector.get(i), Token::COMMA) {
                                        println!("comma found after closing parenthesis, an other row of data has to be inserted");
                                        i += 1;
                                        if check_next_token(
                                            token_vector.get(i),
                                            Token::OPEN_PARENTHESIS,
                                        ) {
                                            println!("opening parenthesis found");
                                            if is_first_row {
                                                is_first_row = false;
                                                current_rows_values = 0;
                                            }
                                            i += 1;
                                            continue;
                                        } else {
                                            return error_insert(
                                                "after comma, an other insert line must come",
                                            );
                                        }
                                    } else {
                                        return error_insert(
                                            "missing semicolon at the end of the statement",
                                        );
                                    }
                                }
                                Some(other_token) => {
                                    return error_insert(
                                        format!(
                                            "invalid token ({other_token:?}) after found value."
                                        )
                                        .as_str(),
                                    );
                                }
                                None => {
                                    return error_insert("statement too shrot, ran out of tokens");
                                }
                            }
                        }
                        _ => {
                            return error_insert(
                                format!("token {found_token:?} isn't implemented as data type")
                                    .as_str(),
                            )
                        }
                    },
                    None => return error_create("INSERT statement too short, went out of bounds"),
                }
            }
        }
        Token::SET => {
            //set syntax
            println!("set syntax used");
            return false; //unimplemented
        }
        Token::OPEN_PARENTHESIS => {
            //uses column order definition
            println!("casual syntax used");
            return false;
        }
        _ => {
            return error_insert("After INSERT INTO `db`, only VALUES, SET, or an opening parenthesis \"(\" can come.");
        }
    }

    true
}

pub fn validate_update(token_vector: &Vec<Token>) -> bool {
    // UPDATE `table_name` -> SET column = value -> WHERE (optional) -> AND / OR (optional, based on WHERE) -> ORDER BY (optional) -> LIMIT (optional);

    false
}

pub fn validate_delete(token_vector: &Vec<Token>) -> bool {
    // DELETE FROM `table_name` -> WHERE (optional) -> AND / OR (optional) -> ORDER BY (optional) -> LIMIT (optional);
    //if there is a WHERE, check for a condition, then check for an AND / OR condition

    false
}

pub fn validate_use(token_vector: &Vec<Token>) -> bool {
    // USE `database_name`;
    if token_vector.len() != 3 {
        return false;
    }
    if token_vector[0] == Token::SELECT {
        match token_vector[1] {
            Token::NAME_UNIT(_) => (), //cannot do this with an if statement...
            _ => return false,
        }
        if token_vector[2] != Token::SEMICOLON {
            return false;
        }
    }

    println!("valid use statement");
    true
}

fn error_drop(msg: &str) -> bool {
    println!("error in \"DROP\" command: {msg}");
    false
}

pub fn validate_drop(token_vector: &Vec<Token>) -> bool {
    if token_vector.len() != 4 {
        //3 tokens + ";"
        return error_drop("command is too short or long to be valid");
    }
    if token_vector[0] != Token::DROP {
        return error_drop("DROP statement doesn't start with DROP");
    }
    if token_vector[1] != Token::TABLE && token_vector[1] != Token::DATABASE {
        return error_drop("DROP isn't followed by type (DATABASE / TABLE)");
    }
    match token_vector[2] {
        Token::NO_BACKTICK_NAME_UNIT(_) | Token::NAME_UNIT(_) => (), //if it's a table / database name with "`" characters or without
        _ => return error_drop("Drop type isn't followed by a name"),
    }
    if !check_next_token(token_vector.get(3), Token::SEMICOLON){
        return  error_drop("command isn't closed");
    }
    println!("valid drop statement");

    true
}

pub fn validate_alter(token_vector: &Vec<Token>) -> bool {
    true
}

pub fn validate_syntax(token_vector: &Vec<Token>) -> bool {
    if token_vector.len() < 1 {
        return false;
    }
    match token_vector[0] {
        Token::CREATE => return validate_create(token_vector),
        Token::SELECT => return validate_select(token_vector),
        Token::INSERT => return validate_insert(token_vector),
        Token::UPDATE => return validate_update(token_vector),
        Token::DELETE => return validate_delete(token_vector),
        Token::USE => return validate_use(token_vector),
        Token::DROP => return validate_drop(token_vector),
        Token::ALTER => return validate_alter(token_vector),

        _ => return false, //if it's not an actual starting SQL statement
    }
}

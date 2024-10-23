use crate::command_parser::Token;

fn error_create() -> bool {
    println!("validation error");
    return false;
}

// fn match_token(vector: &Vec<Token>, idx: usize, matched_token: Token){ //would be too slow compared to just checking the token's length
//     match token
// }

fn check_next_token(checked_token: Option<&Token>, check_against: Token) -> bool { //checks if a vector.get() element even exists, and then if it's the right token
    match checked_token {
        Some(token) => return token == &check_against,
        None => return false,
    }
}

fn validate_create(token_vector: &Vec<Token>) -> bool {
    //CREATE -> DATABASE -> `database_name` -> COLLATE -> "collation" (optional) -> CHARACTER -> SET -> "charset" (optional);
    //CREATE -> TABLE -> `table_name` -> (column_name TYPE [optional args], column_name TYPE [optional args]); 
    println!("validating create statement");

    let tokens_len = token_vector.len();
    if tokens_len < 4 || token_vector[0] != Token::CREATE {
        println!("Wrong first token or wrong command length: {:?}", token_vector);
        return false;
    }

    match &token_vector[2]{
        Token::NAME_UNIT(string) => println!("Name unit is {string}"),
        _ => return error_create(),
    }

    if token_vector[1] == Token::DATABASE {
        for mut i in 3..tokens_len {
            if token_vector[i] == Token::SEMICOLON {
                println!("semicolon encountered");
                return true; //if it's a semicolon, return true
            }
            if token_vector[i] == Token::COLLATE {
                println!("collate keyword encountered");
                match token_vector.get(i+1) {
                    Some(found_token) => {
                        match found_token {
                            Token::STRING_VALUE(string_val) => println!("String value: {string_val}"),
                            _ => return error_create(),
                        }
                    },
                    _ => return error_create(), //if the token is not a string
                }
                i += 1; //the next token is a complementer token for "COLLATE", which is checked, so no need to return
            }
            if token_vector[i] == Token::CHARACTER {
                println!("character keyword encountered");
                match token_vector.get(i+1) {
                    Some(found_token) => {
                        match found_token {
                            Token::SET => println!("\"SET\" keyword encountered"), //if the found token is a string token
                            _ => return error_create(),
                        }
                    },
                    None => return error_create(), //if the token is not a string
                }
                match token_vector.get(i+2) {
                    Some(found_token) => {
                        match found_token {
                            Token::STRING_VALUE(string_val) => println!("String value: {string_val}"), //if the found token is a string token
                            _ => return error_create(),
                        }
                    },
                    _ => return error_create(), //if the token is not a string
                }
                i += 2;
            }
            else {
                return error_create(); //if the token after the unit naming token isn't a CHARACTER SET or a COLLATE, false should be returned;
            }
        }
        

    }
    //TODO: this
    else if token_vector[1] == Token::TABLE {
        if token_vector[3] != Token::OPEN_PARENTHESIS {
            return error_create();
        }
        for mut i in 4..tokens_len {
            if token_vector[i] == Token::SEMICOLON {
                println!("semicolon encountered");
                return true; //if it's a semicolon, return true
            }
        
            match token_vector[i] {
                Token::NO_BACKTICK_NAME_UNIT(_) => {
                    if i+1 >= tokens_len { //easy checking implemented, so the code has less match statements
                        return error_create(); //vector would be out of bounds
                    }
                    match token_vector[i+1] {
                        Token::TYPE_KEYWORD(_) => {
                            //if it's a type, it can have a NOT NULL after it
                            if check_next_token(token_vector.get(i+2), Token::NOT){
                                if check_next_token(token_vector.get(i+3), Token::NULL){
                                    i += 3;
                                }
                                else {
                                    return error_create();
                                }
                            }
      
                            //TODO: check the comma after the end of possible keywords, no matter the keyword

                        }
                        _ => return error_create()
                    }
                }
                
                _ => return error_create(),
            }
        }
    }
    

    println!("valid create statement.\n");
    true
}

fn validate_select(token_vector: &Vec<Token>) -> bool {
    // SELECT -> COLUMNS | * -> FROM `table_name` -> WHERE (optional) -> AND / OR (if there's an other criteria)
        // -> GROUP BY (optional) -> HAVING (optional) -> ORDER BY (optional) -> LIMIT (optional);
    false
}

fn validate_insert(token_vector: &Vec<Token>) -> bool {
    // INSERT INTO `table_name` -> (columns) (optional) -> VALUES (values_list) -> ON DUPLICATE KEY UPDATE (optional);

    false
}

fn validate_update(token_vector: &Vec<Token>) -> bool {
    // UPDATE `table_name` -> SET column = value -> WHERE (optional) -> AND / OR (optional, based on WHERE) -> ORDER BY (optional) -> LIMIT (optional);

    false
}

fn validate_delete(token_vector: &Vec<Token>) -> bool {
    // DELETE FROM `table_name` -> WHERE (optional) -> AND / OR (optional) -> ORDER BY (optional) -> LIMIT (optional);
    //if there is a WHERE, check for a condition, then check for an AND / OR condition



    false
}

fn validate_use(token_vector: &Vec<Token>) -> bool {
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


fn error_drop(condition: i32) -> bool{
    println!("error dropping database at condition {condition}");
    false
}

fn validate_drop(token_vector: &Vec<Token>) -> bool {
    if token_vector.len() < 4 { //3 tokens + ";"
        return error_drop(1);
    }
    if token_vector[0] != Token::DROP {
        return error_drop(2);
    }
    if token_vector[1] != Token::TABLE && token_vector[1] != Token::DATABASE {
        return error_drop(3);
    }
    match token_vector[2] {
        Token::NO_BACKTICK_NAME_UNIT(_) | Token::NAME_UNIT(_) => (), //if it's a table / database name with "`" characters or without
        _ => return error_drop(4),

    }
    println!("valid drop statement");

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

        _ => return false //if it's not an actual starting SQL statement
    }

}
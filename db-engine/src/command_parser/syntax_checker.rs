use crate::command_parser::Token;

fn error_create() -> bool {
    println!("validation error");
    return false;
}

fn validate_create(token_vector: &Vec<Token>) -> bool {
    //CREATE -> DATABASE -> `database_name` -> COLLATE -> "collation" (optional) -> CHARACTER -> SET -> "charset" (optional);
    //CREATE -> TABLE -> `table_name` -> (column_name TYPE [optional args], column_name TYPE [optional args]); 
    println!("validating create statement");

    let tokens_len = token_vector.len();
    if tokens_len < 4 || token_vector[0] != Token::CREATE || tokens_len > 9 {
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
                match token_vector.iter().nth(i+1) {
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
                match token_vector.iter().nth(i+1) {
                    Some(found_token) => {
                        match found_token {
                            Token::SET => println!("\"SET\" keyword encountered"), //if the found token is a string token
                            _ => return error_create(),
                        }
                    },
                    _ => return error_create(), //if the token is not a string
                }
                match token_vector.iter().nth(i+2) {
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

    false
}

fn validate_use(token_vector: &Vec<Token>) -> bool {
    // USE `database_name`;
    if token_vector.len() != 2 {
        return false;
    }
    if token_vector[0] == Token::SELECT {
        match token_vector[1] {
            Token::NAME_UNIT(_) => return true, //cannot do this with an if statement...
            _ => return false,
        }
    }
    false
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

        _ => return false //if it's not an actual starting SQL statement
    }

}
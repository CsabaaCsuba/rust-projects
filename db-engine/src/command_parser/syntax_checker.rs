use crate::command_parser::Token;

fn error_create(msg: &str) -> bool {
    println!("create statement: validation error: {msg}");
    return false;
}

// fn match_token(vector: &Vec<Token>, idx: usize, matched_token: Token){ //would be too slow compared to just checking the token's length
//     match token
// }

pub fn check_next_token(checked_token: Option<&Token>, check_against: Token) -> bool { //checks if a vector.get() element even exists, and then if it's the right token
    match checked_token {
        Some(token) => return token == &check_against,
        None => return false,
    }
}

pub fn validate_create(token_vector: &Vec<Token>) -> bool {
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
        _ => return error_create("incorrect name for the database"),
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
                            _ => return error_create("collation isn't specified properly"),
                        }
                    },
                    _ => return error_create("collation isn't specified"), //if the token is not a string
                }
                i += 1; //the next token is a complementer token for "COLLATE", which is checked, so no need to return
            }
            if token_vector[i] == Token::CHARACTER {
                println!("character keyword encountered");
                match token_vector.get(i+1) {
                    Some(found_token) => {
                        match found_token {
                            Token::SET => println!("\"SET\" keyword encountered"), //if the found token is a string token
                            _ => return error_create("CHARACTER keyword must be followed by SET"),
                        }
                    },
                    None => return error_create("nothing found after CHARACTER keyword"), //if the token is not a string
                }
                match token_vector.get(i+2) {
                    Some(found_token) => {
                        match found_token {
                            Token::STRING_VALUE(string_val) => println!("String value: {string_val}"), //if the found token is a string token
                            _ => return error_create("character set is not valid"),
                        }
                    },
                    _ => return error_create("character set was not specified"), //if the token is not a string
                }
                i += 2;
            }
            else {
                return error_create("unrelated keyword was used"); //if the token after the unit 
                                                            //naming token isn't a CHARACTER SET or a COLLATE, false should be returned;
            }
        }
        

    }
    //TODO: write a program which validates create table commands
    else if token_vector[1] == Token::TABLE {
        if token_vector[3] != Token::OPEN_PARENTHESIS {
            return error_create("type specification part is not found");
        }
        let mut base_pattern = false; //whether if the base_pattern was encountered or not
        let mut i = 4;
        while i < tokens_len {
            if token_vector[i] == Token::CLOSING_PARENTHESIS { //in this type of statements, the closing parenthesis also marks the end of the statement basically
                if check_next_token(token_vector.get(i+1), Token::SEMICOLON){
                    if base_pattern { //if colon was definied after a base pattern (because if it wasn't, an unnecessary comma was put into the command)
                        println!("semicolon encountered");
                        return true; //if it's a semicolon, return true
                    }
                    else {
                        return error_create("unnecessary \",\" before the closing parenthetis \")\"");
                    }
                    
                }
                else {
                    
                    return error_create("statement isn't closed properly");
                }
                
            }
        
            match token_vector[i] {
                Token::NO_BACKTICK_NAME_UNIT(_) => {
                    // if i+1 >= tokens_len { //easy checking implemented, so the code has less match statements
                    //     return error_create(""); //vector would be out of bounds
                    // }
                    match token_vector.get(i+1) {
                        Some(Token::TYPE_KEYWORD(_)) => { //THIS MIGHT NOT WORK
                            println!("type keyword found");
                            //if it's a type, then optional keywords can come like PRIMARY KEY or NOT NULL
                            base_pattern = true;
                            i += 2;
                            // continue;
      
                            //TODO: check the comma after the end of possible keywords, no matter the keyword
                        }
                        _ => return error_create("wrong type keyword after field name specification")
                    }
                }
                //the comma resets the base pattern, because in the next loop, basic tokens can come as well
                Token::COMMA => {
                    if base_pattern {
                        println!("column definition ended. next column...");
                        base_pattern = false;
                        i += 1;
                    }
                    else {
                        return error_create("comma at wrong space");
                    }
                }
                //todo: make this part of the code more concise
                Token::NOT => {
                    if base_pattern {
                        println!("base pattern was already found. NOT keyword is legitimate");
                        if check_next_token(token_vector.get(i+1), Token::NULL){
                            println!("after NOT, NULL token is there");
                            i += 2; //if the next token is NULL, it's valid, but nothing has to be done apart from incrementing the loop
                        }
                        else {
                            return error_create("NOT keyword must be followed by NULL");
                        }
                    }
                    else {
                        return error_create("NOT NULL must be used after field name and type");
                    }
                }
                Token::PRIMARY => {
                    if base_pattern {
                        println!("base pattern was already found. PRIMARY keyword is legitimate");
                        if check_next_token(token_vector.get(i+1), Token::KEY){
                            println!("after PRIMARY, KEY token is there");
                            i += 2; //if the next token is NULL, it's valid, but nothing has to be done apart from incrementing the loop
                        }
                        else {
                            return error_create("PRIMARY must be followed by KEY");
                        }
                    }
                    else {
                        return error_create("PRIMARY KEY must be after field name and type");
                    }
                }
                Token::AUTO_INCREMENT => {
                    if base_pattern {
                        println!("auto increment token was found in it's rightful place");
                        i += 1;
                    }
                    else {
                        return error_create("AUTO_INCREMENT was used in the bad place");
                    }

                }
                //TODO: Add other optional keywords here


                
                _ => return error_create(format!("wrong keyword in create table statement: {:?}", token_vector[i]).as_str()),
            }
        }
    }
    

    println!("valid create statement.\n");
    true
}

pub fn validate_select(token_vector: &Vec<Token>) -> bool {
    // SELECT -> COLUMNS | * -> FROM `table_name` -> WHERE (optional) -> AND / OR (if there's an other criteria)
        // -> GROUP BY (optional) -> HAVING (optional) -> ORDER BY (optional) -> LIMIT (optional);
    false
}

fn error_insert(error_msg: &str) -> bool{
    println!("Error inserting: {error_msg}");
    return false;
}

pub fn validate_insert(token_vector: &Vec<Token>) -> bool {
    // INSERT INTO `table_name` -> (columns) (optional) -> VALUES (values_list) -> ON DUPLICATE KEY UPDATE (optional);
    //TODO: write the longer variant (column definitions as well)
    let tokens_len = token_vector.len();
    if tokens_len < 6 {
        return false;
    }
    //checking the starting - default cases
    if !check_next_token(token_vector.get(0), Token::INSERT){
        return error_insert("the first token is not an insert token");
    }
    if !check_next_token(token_vector.get(1), Token::INTO){
        return error_insert("the second token is not \"INTO\"");
    }
    match token_vector.get(2){
        Some(token)=> {
            match token {
                Token::NO_BACKTICK_NAME_UNIT(_) => (),
                Token::NAME_UNIT(_) => (),
                _ => return error_insert("table name incorrect."),
            }
        },
        None => return error_insert("sentence too short"),
    }
    if !check_next_token(token_vector.get(3), Token::OPEN_PARENTHESIS){
        return error_insert("opening parenthesis for the columns not found");
    }
    //checking the data types
    let mut columns_included = 0;
    let mut data_included = 0;
    for mut i in 4..tokens_len {

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


fn error_drop(condition: i32) -> bool{
    println!("error dropping database at condition {condition}");
    false
}

pub fn validate_drop(token_vector: &Vec<Token>) -> bool {
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
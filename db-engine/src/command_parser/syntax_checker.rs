use crate::command_parser::Token;


 //for parsing general select statements, for select, update, delete
fn parse_general_select(vector_part: &&Vec<Token>) -> bool{ //takes in a slice of a borrowed vector

    false
}

fn validate_create(token_vector: &Vec<Token>) -> bool {
    //CREATE -> DATABASE -> `database_name` -> COLLATE (optional) -> CHARACTER SET "charset" (optional);
    //CREATE -> TABLE -> `table_name` -> (column_name TYPE [optional args], column_name TYPE [optional args]); 


    false
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
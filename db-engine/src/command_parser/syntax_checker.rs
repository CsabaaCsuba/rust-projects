use crate::command_parser::Token;

fn validate_create(token_vector: &Vec<Token>) -> bool {
    //CREATE -> DATABASE | TABLE `item_name` -> COLLATE (optional) -> CHARACTER SET "charset" (optional);
}

fn validate_select(token_vector: &Vec<Token>) -> bool {
    // SELECT -> COLUMNS | * -> FROM `table_name` -> WHERE (optional) -> GROUP BY (optional) -> HAVING (optional) -> ORDER BY (optional) -> LIMIT (optional);

}

fn validate_insert(token_vector: &Vec<Token>) -> bool {

}

fn validate_update(token_vector: &Vec<Token>) -> bool {

}

fn validate_delete(token_vector: &Vec<Token>) -> bool {

}

fn validate_use(token_vector: &Vec<Token>) -> bool {

}

pub fn validate_syntax(token_vector: &Vec<Token>) -> bool{

    

    true
}
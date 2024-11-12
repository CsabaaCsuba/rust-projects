//this will convert token arrays to easily "digestible" data for the database engine
use std::{env::current_exe, mem::discriminant};


use super::tokens::Token;

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

impl DBCreationData {
    //the validity checking will happen here. Since it can fail, it has to be unwrapped upon calling
    fn from_tokens(token_vector: Vec<Token>) -> Option<Self> { 

    }
}


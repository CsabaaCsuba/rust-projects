use std::{fmt::Error, io};

mod commands;
use commands::handle_found_statement;

use crate::commands::match_statement_type;
/* 
things to implement after CREATE:
DATABASE 'database-name' CHARACTER SET "utf8" collate "utf8_hungarian_ci";
TABLE
INDEX
VIEW
*/




fn main() {
    let mut input = String::new();
    loop {
        println!("input: ");
        if let Ok(bytes_read) = io::stdin().read_line(&mut input) { //.read_line adds to the string, it does not rewrite it
            println!("bytes read: {bytes_read}");
            let keyword_parse_result = match_statement_type(&mut input);
            
            let first_keyword = match keyword_parse_result {
                Ok(key) => key,
                Err(Error) => {
                    println!("first keyword is invalid");
                    continue;
                }
            };

            println!("First keyword is {first_keyword:?}");


            handle_found_statement(&mut input);


            input.clear();
        }
        else {
            println!("failed to read input."); //cannot access the error message this way...
        }
    }
    
    
}

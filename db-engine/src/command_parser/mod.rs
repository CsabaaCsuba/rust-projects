pub mod tokens;
pub mod tokenizer;
use tokenizer::*;
use tokens::Token;
pub mod syntax_checker; //public makes it available to the other parts of the program
use syntax_checker::validate_syntax;

use std::io;

// static POSSIBLE_TOKENS: [&str; 8] = [
//     "CREATE", "SELECT", "INSERT", "UPDATE", "DELETE", "ALTER", "TABLE", "DATABASE",
// ];



pub fn read_command_loop() {
    let mut command = String::with_capacity(128);
    let mut input = String::with_capacity(128); //memory is enough for shorter commands
    loop {
        //while the command is not closed properly
        while !compare_last_char(&input, ';') {
            io::stdin().read_line(&mut input).unwrap(); //input reading shouldn't really fail
            command.push_str(&input);
        }

        let parsed_tokens = parse_command(&command);
        if parsed_tokens.capacity() == 0 {
            //if the command fails, an error message is printed -> only a jump to the next iteration is needed
            continue;
        }
        if validate_syntax(&parsed_tokens) {
            println!("syntax is valid.");
            //if syntax validation succeeded
        }

        command.clear();
        input.clear();
    }
}

pub mod handler;
pub mod parser;
pub mod tokenizer;
pub mod tokens;
use std::{io, thread};
use tokenizer::*;

pub fn read_command_loop() {
    
    let mut command = String::with_capacity(128);
    let mut input = String::with_capacity(128); //memory is enough for shorter commands
    loop {
        //while the command is not closed properly
        while !compare_last_char(&input, ';') {
            io::stdin().read_line(&mut input).unwrap(); //input reading shouldn't really fail
            command.push_str(&input);
        }

        let tokens = parse_command(&command);
        if tokens.capacity() == 0 {
            //if the command fails, an error message is printed -> only a jump to the next iteration is needed
            println!("Parsing command failed.\n");
            continue;
        }

        handler::infer_op(&tokens);



        command.clear();
        input.clear();
    }

}

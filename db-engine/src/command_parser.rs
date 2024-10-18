mod tokens;
use tokens::Token;

use std::io;

static POSSIBLE_TOKENS: [&str; 8] = [
    "CREATE", "SELECT", "INSERT", "UPDATE", "DELETE", "ALTER", "TABLE", "DATABASE",
];

fn parse_next_command_part(command_part: &str) -> Option<Token> {
    let token: Token;

    //match the command_part with all of the POSSIBLE_TOKENS with a for loop, then

    match command_part {
        "CREATE" => token = Token::CREATE,
        "SELECT" => token = Token::SELECT,
        "INSERT" => token = Token::INSERT,
        "UPDATE" => token = Token::UPDATE,
        "DELETE" => token = Token::DELETE,
        "ALTER" => token = Token::ALTER,
        "TABLE" => token = Token::TABLE, 
        "DATABASE" => token  = Token::DATABASE,

        _ => return None,
    }

    Some(token)
}

fn parse_command(command: &String) -> Vec<Token> {
    //TODO: remove newlines / whitespaces, so everything is only separated by spaces
    let mut parsed_tokens: Vec<Token> = Vec::new();
    let command_parts: Vec<&str> = command.split_whitespace().collect();

    for word in command_parts {
        match parse_next_command_part(word) {
            Some(token_value) => parsed_tokens.push(token_value),
            None => {
                println!("parsing token failed at word: {word:?}");
                parsed_tokens.clear();
                parsed_tokens.shrink_to(0);
                return parsed_tokens; //returns empty vector
            }
        }
    }

    parsed_tokens
}

pub fn read_command_loop() {
    let mut input = String::with_capacity(128); //memory is enough for shorter commands
    loop {
        io::stdin().read_line(&mut input).unwrap(); //input reading shouldn't really fail

        let parsed_tokens = parse_command(&input);
        if parsed_tokens.capacity() == 0 {
            //if the command fails, an error message is printed -> only a jump to the next iteration is needed
            continue;
        }
    }
}

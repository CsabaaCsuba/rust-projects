mod tokens;
use tokens::Token;
mod syntax_checker;
use syntax_checker::validate_syntax;

use std::io;

static POSSIBLE_TOKENS: [&str; 8] = [
    "CREATE", "SELECT", "INSERT", "UPDATE", "DELETE", "ALTER", "TABLE", "DATABASE",
];

fn parse_next_command_part(command_part: &str) -> Option<Token> {
    let token: Token;

    let first_char = command_part.chars().nth(0).unwrap(); //safe to unwrap here, outer function doesn't allow empty command part string slice
    

    if first_char == '`' {
        let cmd_part_len = command_part.len();

        let last_char = command_part.chars().nth(cmd_part_len -1).unwrap();
        if last_char != '`' {
            return None;
        }
        let token_value = command_part.get(1..cmd_part_len).unwrap();

        let return_value = token_value.to_string();
        return Some(Token::NAME_UNIT(return_value));

    }
    else if first_char == '"' {
        let cmd_part_len = command_part.len();

        let last_char = command_part.chars().nth(cmd_part_len -1).unwrap();
        if last_char != '"' {
            return None;
        }
        let token_value = command_part.get(1..cmd_part_len).unwrap();

        let return_value = token_value.to_string();
        return Some(Token::STRING_VALUE(return_value));
    }
    else if (first_char as u8) >= 48 && (first_char as u8) <= 57 {
        let numeric_value = command_part.parse::<f64>().unwrap();
        if (numeric_value as i32) as f64 == numeric_value { //if numeric value is a whole number
            return Some(Token::INT_VALUE(numeric_value as i32));
        }
        else {
            return Some(Token::DOUBLE_VALUE(numeric_value));
        }
    }


    let experimental_str = command_part.to_owned().to_string();
    let uppercase_cmd = experimental_str.to_uppercase();
    let uppercase_str = uppercase_cmd.as_str();
    //match the command_part with all of the POSSIBLE_TOKENS with a for loop, then
    match uppercase_str {
        "FALSE" => token = Token::BOOL_VALUE(false),
        "TRUE" => token = Token::BOOL_VALUE(true),

        "CREATE" => token = Token::CREATE,
        "SELECT" => token = Token::SELECT,
        "INSERT" => token = Token::INSERT,
        "UPDATE" => token = Token::UPDATE,
        "DELETE" => token = Token::DELETE,
        "ALTER" => token = Token::ALTER,
        "DROP" => token = Token::DROP,
        "USE" => token = Token::USE,


        "TABLE" => token = Token::TABLE,
        "DATABASE" => token = Token::DATABASE,

        "COLLATE" => token = Token::COLLATE,
        "CHARACTER" => token = Token::CHARACTER,
        "SET" => token = Token::SET,

        "<" => token = Token::SMALLER_THAN_OPERATOR,
        ">" => token = Token::BIGGER_THAN_OPERATOR,
        "=" => token = Token::EQUAL_EXPRESSION,

        "FROM" => token = Token::FROM,
        "WHERE" => token = Token::WHERE,
        "AND" => token = Token::AND,
        "OR" => token = Token::OR,
        "GROUP" => token = Token::GROUP,
        "BY" => token = Token::BY,
        "ORDER" => token = Token::ORDER,
        "HAVING" => token = Token::HAVING,
        "LIMIT" => token = Token::LIMIT,




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
        if validate_syntax(&parsed_tokens){
            //if syntax validation succeeded
        }
    }
}

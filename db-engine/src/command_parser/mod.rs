pub mod tokens;
use tokens::{Token, TypeNotation};
pub mod syntax_checker; //public makes it available to the other parts of the program
use syntax_checker::validate_syntax;

use std::io;

// static POSSIBLE_TOKENS: [&str; 8] = [
//     "CREATE", "SELECT", "INSERT", "UPDATE", "DELETE", "ALTER", "TABLE", "DATABASE",
// ];

fn parse_next_command_part(command_part: &str) -> Option<Token> {
    let token: Token;

    let first_char = command_part.chars().nth(0).unwrap(); //safe to unwrap here, outer function doesn't allow empty command part string slice

    if first_char == '`' {
        let cmd_part_len = command_part.len();

        let last_char = command_part.chars().nth(cmd_part_len - 1).unwrap();
        if last_char != '`' {
            return None;
        }
        let token_value = command_part.get(1..cmd_part_len - 1).unwrap();

        let return_value = token_value.to_string();
        return Some(Token::NAME_UNIT(return_value));
    } else if first_char == '"' {
        let cmd_part_len = command_part.len();

        let last_char = command_part.chars().nth(cmd_part_len - 1).unwrap();
        if last_char != '"' {
            return None;
        }
        let token_value = command_part.get(1..cmd_part_len - 1).unwrap();

        let return_value = token_value.to_string();
        return Some(Token::STRING_VALUE(return_value));
    } else if (first_char as u8) >= 48 && (first_char as u8) <= 57 {
        let numeric_value = command_part.parse::<f64>().unwrap();
        if (numeric_value as i32) as f64 == numeric_value {
            //if numeric value is a whole number
            return Some(Token::INT_VALUE(numeric_value as i32));
        } else {
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

        "INTO" => token = Token::INTO,

        "TABLE" => token = Token::TABLE,
        "DATABASE" => token = Token::DATABASE,

        "COLLATE" => token = Token::COLLATE,
        "CHARACTER" => token = Token::CHARACTER,
        "SET" => token = Token::SET,

        "<" => token = Token::SMALLER_THAN_OPERATOR,
        ">" => token = Token::BIGGER_THAN_OPERATOR,
        "=" => token = Token::EQUAL_EXPRESSION,

        "(" => token = Token::OPEN_PARENTHESIS,
        ")" => token = Token::CLOSING_PARENTHESIS,
        "," => token = Token::COMMA,

        ";" => token = Token::SEMICOLON,

        "*" => token = Token::ASTERISK,

        "FROM" => token = Token::FROM,
        "WHERE" => token = Token::WHERE,
        "AND" => token = Token::AND,
        "OR" => token = Token::OR,
        "GROUP" => token = Token::GROUP,
        "BY" => token = Token::BY,
        "ORDER" => token = Token::ORDER,
        "HAVING" => token = Token::HAVING,
        "LIMIT" => token = Token::LIMIT,

        "VALUES" => token = Token::VALUES,

        "DOUBLE" => token = Token::TYPE_KEYWORD(TypeNotation::DOUBLE),
        "INT" => token = Token::TYPE_KEYWORD(TypeNotation::INT),
        "TEXT" => token = Token::TYPE_KEYWORD(TypeNotation::TEXT),
        "BOOL" => token = Token::TYPE_KEYWORD(TypeNotation::BOOL),

        "PRIMARY" => token = Token::PRIMARY,
        "KEY" => token = Token::KEY,
        "NOT" => token = Token::NOT,
        "NULL" => token = Token::NULL,
        "UNIQUE" => token = Token::UNIQUE,
        "AUTO_INCREMENT" => token = Token::AUTO_INCREMENT,
        "DEFAULT" => token = Token::DEFAULT,

        "CONSTRAINT" => token = Token::CONSTRAINT,

        "FOREIGN" => token = Token::FOREIGN,
        "REFERENCES" => token = Token::REFERENCES,

        "ON" => token = Token::ON,

        _ => token = Token::NO_BACKTICK_NAME_UNIT(experimental_str), //experimental_str is consumed here, but it's no issue
    }

    Some(token)
}

pub fn parse_command(command: &String) -> Vec<Token> {
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

//UPDATE: this solution is shit
//check if the last non-whitespace character of the string equals the right_character
fn compare_last_char(checked_string: &String, right_character: char) -> bool {
    let string_parts: Vec<&str> = checked_string.split_whitespace().collect();
    let string_parts_len = string_parts.len();
    if string_parts_len == 0 {
        return false;
    }
    if string_parts[string_parts_len - 1] == right_character.to_string() {
        return true;
    }
    let last_str_len = string_parts[string_parts_len - 1].len();
    if string_parts[string_parts_len - 1]
        .chars()
        .nth(last_str_len - 1)
        .unwrap()
        == right_character
    {
        return true;
    }

    false
}

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
            //if syntax validation succeeded
        }

        command.clear();
        input.clear();
    }
}

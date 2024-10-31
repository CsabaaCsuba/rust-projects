use super::tokens::{StringWrapper, Token, TypeNotation};

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
        return Some(Token::NAME_UNIT(StringWrapper::Data(return_value)));
    } else if first_char == '"' {
        let cmd_part_len = command_part.len();

        let last_char = command_part.chars().nth(cmd_part_len - 1).unwrap();
        if last_char != '"' {
            return None;
        }
        let token_value = command_part.get(1..cmd_part_len - 1).unwrap();

        let return_value = token_value.to_string();
        return Some(Token::STRING_VALUE(StringWrapper::Data(return_value)));
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
        "=" => token = Token::EQUAL_SIGN,

        "(" => token = Token::OPEN_PARENTHESIS,
        ")" => token = Token::CLOSING_PARENTHESIS,
        "," => token = Token::COMMA,

        ";" => token = Token::SEMICOLON,

        "IS" => token = Token::IS,
        "LIKE" => token = Token::LIKE,

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

        "ASC" => token = Token::ASC,
        "DESC" => token = Token::DESC,

        "VALUES" => token = Token::VALUES,

        "UNION" => token = Token::UNION,

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

        "CASCADE" => token = Token::CASCADE,
        "NO" => token = Token::NO,
        "ACTION" => token = Token::ACTION,
        "RESTRICT" => token = Token::RESTRICT,

        _ => token = Token::NO_BACKTICK_NAME_UNIT(StringWrapper::Data(experimental_str)), //experimental_str is consumed here, but it's no issue
    }

    Some(token)
}

#[allow(non_camel_case_types)]
enum number_type {
    INT(i32),
    DOUBLE(f64),
    NaN,
}

fn str_to_num(command_string: &[u8], idx: usize, number_ptr: &mut number_type) -> usize {
    //TODO: write this function
    //returns the new idx value
    let parsed_num: number_type; //it gets value once, so it doesn't have to be mutable
    let mut currently_parsed: f64 = 0.0;
    let mut i = idx;
    let mut is_double = false;
    let mut j: f64 = 1.0;
    while i < command_string.len() {
        if command_string[i] >= 48 && command_string[i] <= 57 {
            if is_double {
                currently_parsed += (command_string[i] - 48) as f64 / (f64::powf(10.0, j));
                j += 1.0;
            } else {
                currently_parsed = currently_parsed * 10.0;
                currently_parsed += (command_string[i] - 48) as f64;
            }
        } else if command_string[i] == b'.' {
            //if number isn't a whole number
            is_double = true;
        } else if command_string[i] == b'\n' //if whitespace / end character reached (it's not incremented because of the end characters' parsing)
            || command_string[i] == b'\r'
            || command_string[i] == b' '
            || command_string[i] == b';'
            || command_string[i] == b','
            || command_string[i] == b'+'
            || command_string[i] == b'-'
            || command_string[i] == b'/'
            || command_string[i] == b'<'
            || command_string[i] == b'>'
            || command_string[i] == b'='
            || command_string[i] == b'*'
            || command_string[i] == b'('
            || command_string[i] == b')'
            || command_string[i] == b'&'
            || command_string[i] == b'|'
            || command_string[i] == b'^' //bitwise xor
            || command_string[i] == b'~'
        //bitwise not
        {
            parsed_num = if is_double {
                number_type::DOUBLE(currently_parsed)
            } else {
                number_type::INT(currently_parsed as i32)
            };
            *number_ptr = parsed_num;
            return i - 1;
        } else {
            println!(
                "illegal character encountered when parsing number: {}",
                command_string[i]
            );
            return 0;
        }

        i += 1;
    }

    i - 1
}

fn parse_until_byte(command_string: &[u8], idx: usize, end_byte: u8) -> String {
    let string_len = command_string.len();
    println!("byte-based string parsing started");
    let mut i = idx;
    let mut return_string = String::new();
    if end_byte == b' ' {
        //put here so it doesn't have to get checked on each iteration
        while i < string_len && command_string[i] != end_byte {
            println!("current byte: {}", command_string[i]);
            //TODO: check for whitespaces / other terminators like ")" or "," or ";" when the end_byte is a space
            match command_string[i] {
                b')' | b',' | b';' => {
                    return return_string;
                }
                _ => (),
            }

            let current_char = unsafe { char::from_u32_unchecked(command_string[i] as u32) };
            println!("current char: {}", current_char);
            return_string.push(current_char);
            i += 1;
        }
    } else {
        while i < string_len && command_string[i] != end_byte {
            println!("current byte: {}", command_string[i]);
            //it can't be unsafe since the IO reader does a UTF8 validity check
            let current_char = unsafe { char::from_u32_unchecked(command_string[i] as u32) };
            println!("current char: {}", current_char);
            return_string.push(current_char);
            i += 1;
        }
    }

    return_string
}

//returns the increased index variable, AND modified the token's memory address
fn parse_next_multi_char_part(command_string: &[u8], idx: usize, token_ptr: &mut Token) -> usize {
    let i: usize = idx; //so it starts from idx
    if command_string[i] == b'"' {
        let return_string = parse_until_byte(command_string, i + 1, b'"');
        let string_len = return_string.len();
        if string_len == 0 {
            println!("string with 0 characters was read");
            return 0;
        }
        *token_ptr = Token::STRING_VALUE(StringWrapper::Data(return_string));

        return idx + string_len + 1; //for the ending character
    } else if command_string[i] == b'`' {
        let return_string = parse_until_byte(command_string, i + 1, b'`');
        let string_len = return_string.len();
        if string_len == 0 {
            println!("no-backtick name unit with 0 characters was read");
            return 0;
        }
        *token_ptr = Token::NAME_UNIT(StringWrapper::Data(return_string));

        return idx + string_len + 1;
    } else if command_string[i] > 48 && command_string[i] < 57 {
        let mut parsed_num: number_type = number_type::NaN;
        let new_idx = str_to_num(command_string, i, &mut parsed_num);
        if i < new_idx {
            //if parsing was succesfull
            match parsed_num {
                number_type::INT(val) => {
                    *token_ptr = Token::INT_VALUE(val);
                }
                number_type::DOUBLE(val) => {
                    *token_ptr = Token::DOUBLE_VALUE(val);
                }
                number_type::NaN => {
                    println!("parser moved forward, but number was not found??");
                    return 0;
                }
            }
            return new_idx;
        } else {
            println!("parsing number failed...");
            return 0;
        }
    } else {
        let found_string = parse_until_byte(command_string, i, b' ');
        let string_len = found_string.len();
        if string_len == 0 {
            println!("parsed string's len was 0");
            return 0;
        }
        let command_token = parse_next_command_part(&found_string);
        match command_token {
            Some(found_token) => {
                *token_ptr = found_token;
                return idx + string_len - 1; //minus 1 so it doesn't go to the next character because of the length addition. In the other cases this is not a problem
            }
            None => {
                println!("keyword not found...");
                return 0;
            }
        }
    }
}

fn parse_entire_command(command: &String) -> Vec<Token> {
    let mut parsed_tokens: Vec<Token> = Vec::new();
    let mut i = 0;
    let command_bytes = command.as_bytes();
    let mut token_space: Token = Token::NO; //some filler has to be there
    while i < command_bytes.len() {
        match command_bytes[i] {
            b'<' => parsed_tokens.push(Token::SMALLER_THAN_OPERATOR),
            b'>' => parsed_tokens.push(Token::BIGGER_THAN_OPERATOR),
            b'=' => parsed_tokens.push(Token::EQUAL_SIGN),
            b'(' => parsed_tokens.push(Token::OPEN_PARENTHESIS),
            b')' => parsed_tokens.push(Token::CLOSING_PARENTHESIS),
            b',' => parsed_tokens.push(Token::COMMA),
            b';' => parsed_tokens.push(Token::SEMICOLON),
            b'*' => parsed_tokens.push(Token::ASTERISK),
            b'\n' | b'\r' | b' ' => (), //if it's a formatter, just continue

            //in case of an other char, if parse was succesfull
            _ => {
                let new_idx = parse_next_multi_char_part(command_bytes, i, &mut token_space);
                println!("new idx: {new_idx}, i: {i}");
                if i < new_idx {
                    //if idx changed, the parsing was succesfull
                    println!("token found: {:?}", &token_space);
                    parsed_tokens.push(token_space.clone());
                    i = new_idx;
                } else {
                    //if parsing failed
                    println!("parsing token failed.\n");
                    parsed_tokens.clear();
                    parsed_tokens.shrink_to(0);
                    return parsed_tokens;
                }
            }
        }
        i += 1; //continue to next character
    }

    parsed_tokens
}

pub fn parse_command(command: &String) -> Vec<Token> {
    //TODO: remove newlines / whitespaces, so everything is only separated by spaces
    let parsed_tokens = parse_entire_command(command);

    parsed_tokens
}

//UPDATE: this solution is shit
//check if the last non-whitespace character of the string equals the right_character
pub fn compare_last_char(checked_string: &String, right_character: char) -> bool {
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

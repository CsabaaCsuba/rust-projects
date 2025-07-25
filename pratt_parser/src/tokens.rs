#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    OpenParenthesis,
    ClosedParenthesis,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next(); // Skip whitespace
            }
            '0'..='9' | '.' => {
                let mut number_str = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        number_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match number_str.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => return Err(format!("Invalid number: {}", number_str)),
                }
            }
            '+' => {
                chars.next();
                tokens.push(Token::Op(Operator::Plus));
            }
            '-' => {
                chars.next();
                tokens.push(Token::Op(Operator::Minus));
            }
            '*' => {
                chars.next();
                tokens.push(Token::Op(Operator::Multiply));
            }
            '/' => {
                chars.next();
                tokens.push(Token::Op(Operator::Divide));
            }
            '(' => {
                chars.next();
                tokens.push(Token::OpenParenthesis);
            }
            ')' => {
                chars.next();
                tokens.push(Token::ClosedParenthesis);
            }
            _ => {
                return Err(format!("Unexpected character: {}", ch));
            }
        }
    }

    Ok(tokens)
}

#[test]
fn test1() {
    let tokens = vec![
        Token::Number(1.0),
        Token::Op(Operator::Plus),
        Token::OpenParenthesis,
        Token::Number(9.0),
        Token::Op(Operator::Minus),
        Token::Number(4.0),
        Token::ClosedParenthesis,
    ];
    assert_eq!(tokens, tokenize("1.0+(9-4)").unwrap());
}
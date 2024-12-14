use std::any::Any;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    IDENTIFIER, STRING, NUMBER,

    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Box<dyn Any>, line: usize,) -> Self {
        Self { token_type, lexeme, literal, line}
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?} {}", self.token_type, self.lexeme, self.literal, self.line)
    }
}
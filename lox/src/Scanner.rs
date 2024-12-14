use crate::{ErrorInfo, SyntaxError, TokenDef::{Token, TokenType}};

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let cloned_str = source.clone().to_owned();
        let new_vec: Vec<Token> = Vec::new();
        let src_chars = cloned_str.chars().collect();
        Self {source: cloned_str, source_chars: src_chars, tokens: new_vec, start: 0, current: 0, line: 1}
    }

    pub fn scan_tokens(&mut self) -> Vec<Token>{
        while !(&self.is_at_end()) {

            self.start = self.current;
            self.scan_tokens();
        }

        todo!("Add Token Vector returner as the function caller wants it...");
    }

    fn is_at_end(&self) -> bool{
        return &self.current >= &self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), SyntaxError>{
        let current_char: char = self.advance();
        match current_char {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            _ => {
                let err_msg = "Unexpected character".to_string();
                eprintln!("Unexpected character");

                return Err(SyntaxError::DescribedError(ErrorInfo::new(self.line, self.current, err_msg)));
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char{
        self.current += 1;
        return *self.source_chars.get(self.current-1)
            .expect("The code had more than 2**64 characters. Congratulations...");
    }

    fn add_token(&mut self, token_type: TokenType) { //the literal might become a parameter later
        let text: String = self.source_chars[self.start..self.current].iter().collect();
        let literal: Box<Option<u8>> = Box::new(None); //I don't know what the literal's type will be yet...
        let new_token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(new_token);
    }

}
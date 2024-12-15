use std::collections::HashMap;

use crate::{ErrorInfo, SyntaxError, TokenDef::{Token, TokenType, LiteralType}};

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("false".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("fun".to_string(), TokenType::FUN);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("nil".to_string(), TokenType::NIL);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("print".to_string(), TokenType::PRINT);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("var".to_string(), TokenType::VAR);
        keywords.insert("while".to_string(), TokenType::WHILE);
        let cloned_str = source.to_owned();
        let new_vec: Vec<Token> = Vec::new();
        let src_chars = cloned_str.chars().collect();
        Self {source: cloned_str, source_chars: src_chars, tokens: new_vec, start: 0, current: 0, line: 1, keywords: keywords}
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
            '(' => self.add_token(TokenType::LEFT_PAREN, LiteralType::None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, LiteralType::None),
            '{' => self.add_token(TokenType::LEFT_BRACE, LiteralType::None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, LiteralType::None),
            ',' => self.add_token(TokenType::COMMA, LiteralType::None),
            '.' => self.add_token(TokenType::DOT, LiteralType::None),
            '-' => self.add_token(TokenType::MINUS, LiteralType::None),
            '+' => self.add_token(TokenType::PLUS, LiteralType::None),
            ';' => self.add_token(TokenType::SEMICOLON, LiteralType::None),
            '*' => self.add_token(TokenType::STAR, LiteralType::None),

            '!' => {
                if self.match_next('='){
                    self.add_token(TokenType::BANG_EQUAL, LiteralType::None);
                }
                else {
                    self.add_token(TokenType::BANG, LiteralType::None);
                }
            },
            '=' => {
                if self.match_next('='){
                    self.add_token(TokenType::EQUAL, LiteralType::None);
                }
                else {
                    self.add_token(TokenType::EQUAL_EQUAL, LiteralType::None);
                }
            },
            '<' => {
                if self.match_next('='){
                    self.add_token(TokenType::LESS_EQUAL, LiteralType::None);
                }
                else {
                    self.add_token(TokenType::LESS, LiteralType::None);
                }
            },
            '>' => {
                if self.match_next('='){
                    self.add_token(TokenType::GREATER_EQUAL, LiteralType::None);
                }
                else {
                    self.add_token(TokenType::GREATER, LiteralType::None);
                }
            },
            '/' => {
                if self.match_next('/'){
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else if self.match_next('*'){
                    loop {
                        let current = self.advance();
                        let next = self.advance();

                        if current == '*' && next == '/' {
                            break; //the multi-line comment is over
                        }
                        else if next == '*' {
                            // if only the next is a *, then the current index has to be
                            // set back by one, so the star can be checked on the next iteration as current
                            self.current -= 1;
                        }
                        else { //if it's not a comment ending, the only important characters are newlines 
                            if current == '\n'{
                                self.line += 1;
                            }
                            if next == '\n' {
                                self.line += 1;
                            }
                        }


                    }
                }
                else {
                    self.add_token(TokenType::SLASH, LiteralType::None);
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            'o' => {
                if self.peek() == 'r'{
                    self.add_token(TokenType::OR, LiteralType::None);
                }
            }
            _ => {
                if current_char.is_numeric() {
                    self.number();
                }
                else if current_char.is_alphabetic() || current_char == '_'{

                }
                else {
                    let err_msg = "Unexpected character".to_string();
                    eprintln!("{err_msg}");
                    return Err(SyntaxError::DescribedError(ErrorInfo::new(self.line, self.current, err_msg)));
                }   
            }
        }
        Ok(())
    }

    fn peek(&mut self) -> char {
        match self.source_chars.get(self.current) {
            Some(ch) => {
                ch.clone()
            },
            _ => '\0',
        }
    }

    fn peek_next(&mut self) -> char {
        match self.source_chars.get(self.current+1) {
            Some(ch) => {
                ch.clone()
            },
            _ => '\0',
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        self.current += 1;
        match self.source_chars.get(self.current) {
            Some(ch) => {
                return ch.clone() == expected;
            },
            _ => false,
        }
    }

    fn advance(&mut self) -> char{
        self.current += 1;
        return *self.source_chars.get(self.current-1)
            .expect("The code had more than 2**64 characters. Congratulations...");
    }

    fn add_token(&mut self, token_type: TokenType, literal_val: LiteralType) { //the literal might become a parameter later
        let text: String = self.source_chars[self.start..self.current].iter().collect();
        let new_token = Token::new(token_type, text, Box::new(literal_val), self.line);
        self.tokens.push(new_token);
    }

    fn string(&mut self) -> Result<(), SyntaxError>{
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end(){
            let err_msg = "Unexpected character".to_string();
            eprintln!("{err_msg}");
            return Err(SyntaxError::DescribedError(ErrorInfo::new(self.line, self.current, err_msg)));
        }

        self.advance(); //to jump over the closing
        let value: String = self.source_chars[self.start+1..self.current-1].iter().collect();
        self.add_token(TokenType::STRING, LiteralType::String(value));


        Ok(())
    }

    fn number(&mut self){
        while self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let numeric_part: String = self.source_chars[self.start..self.current].iter().collect();

        let num = numeric_part.parse::<f64>()
            .expect("This cannot really happen?");

        self.add_token(TokenType::NUMBER, LiteralType::Number(num));
    }

    fn identifier(&mut self){
        loop {
            let current_char = self.peek();
            if current_char.is_alphanumeric() || current_char == '_' {
                self.advance();
            }
            else {
                break;
            }
        }

        let text: String = self.source_chars[self.start..self.current].iter().collect();
        let token_type = self.keywords.get(&text);
        match token_type {
            //if it's in the hashmap, return the right variable name. Else, it's an identifier (variable name)
            Some(val) => self.add_token(val.to_owned(), LiteralType::None),
            None => self.add_token(TokenType::IDENTIFIER, LiteralType::None)
        }
    }


}
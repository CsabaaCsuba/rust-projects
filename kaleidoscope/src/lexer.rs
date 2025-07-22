use std::str::SplitWhitespace;
use std::fmt::Debug;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'src> {
    FuncDef = 1,
    Extern = 2,

    Identifier(&'src str) = 3,
    Number(f64) = 4,

    Operator(Ops) = 5,
    OpenParen = 6,
    ClosedParen = 7,
    Comma = 8,
    Semicolon = 9,

    Unknown(&'src str) = 10,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ops {
    Plus = 0,
    Minus = 1,
    Mult = 2,
    Div = 3
}



#[derive(Debug)]
pub struct Tokens<'src, I> {
    iter: I,
    leftover_slice: Option<&'src str>,
}


pub trait Lex {
    fn lex(&self) -> Tokens<SplitWhitespace>;
}

impl Lex for str {
    fn lex(&self) -> Tokens<SplitWhitespace> {
        Tokens::new(self.split_whitespace())
    }
}

impl <'src, I> Tokens<'src, I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            leftover_slice: None,
        }
    }
}

impl<'src> Token<'src> {
    pub fn is_single_char_token(c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | ';' | ',' | '(' | ')' | '<' | '>' | '|' | '&' | '^' | ':' | '!' => {
                true
            }

            _ => false,
        }
    }
}



impl<'src, I> Iterator for Tokens<'src, I>
where 
    I: Iterator<Item = &'src str>,
{
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut slice = self.leftover_slice
            .take()
            .or_else(|| self.iter.next())?;

        if slice.len() > 1 {
            if let Some(pos) = slice.find(Token::is_single_char_token){
                if pos != 0 {
                    let (immed, rest) = slice.split_at(pos);
                    slice = immed;
                    self.leftover_slice.replace(rest);
                } else {
                    let (immed, rest) = slice.split_at(1);
                    slice = immed;
                    self.leftover_slice.replace(rest);
                }
            }
        }

        Some(tokenize(slice))
    }
}

pub fn tokenize(string: &str) -> Token {
    use Token::*;

    match string {
        "def" => FuncDef,
        "extern" => Extern,

        "+" => Operator(Ops::Plus),
        "-" => Operator(Ops::Minus),
        "*" => Operator(Ops::Mult),
        "/" => Operator(Ops::Div),

        "(" => OpenParen,
        ")" => ClosedParen,

        "," => Comma,
        ";" => Semicolon,

        // Everything else
        text => {
            if let Ok(num) = text.parse::<f64>() {
                Number(num)
            } else {
                if text.chars().nth(0).unwrap().is_alphabetic() {
                    Identifier(text)
                } else {
                    Unknown(text)
                }
            }
        }
    }
}


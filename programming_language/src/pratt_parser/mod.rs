// mod tokens;
// use tokens::*;
use crate::{pratt_parser::primitive_impl::PrimitiveNumber, tokens::{Delimiter, Keyword, Operator, Token}};
use core::num;
use std::fmt::{self, format};

mod primitive_impl;

#[derive(Debug, Clone, PartialEq)]
pub enum NumericValue {
    Char(i8),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f32),
    Double(f64),
}

impl fmt::Display for NumericValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NumericValue::Char(i) => write!(f, "{}", i as i8)?,
            NumericValue::Short(i) => write!(f, "{}", i as i16)?,
            NumericValue::Int(i) => write!(f, "{}", i as i32)?,
            NumericValue::Long(i) => write!(f, "{}", i as i64)?,
            NumericValue::UChar(i) => write!(f, "{}", i as u8)?,
            NumericValue::UShort(i) => write!(f, "{}", i as u16)?,
            NumericValue::UInt(i) => write!(f, "{}", i as u32)?,
            NumericValue::ULong(i) => write!(f, "{}", i as u64)?,
            NumericValue::Float(i) => write!(f, "{}", i as f32)?,
            NumericValue::Double(i) => write!(f, "{}", i as f64)?,
        }
        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(NumericValue),
    Variable(String),
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub struct ExprParser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl ExprParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn parse_expression(&mut self, rbp: u8) -> Result<Expr, String> {
        let mut left = self.nud()?;

        while !self.is_at_end() {
            if let Some(current_token) = self.current() {
                if let Some(bp) = self.lbp(current_token.clone()) {
                    if bp <= rbp {
                        break;
                    }
                    
                    let op = current_token;
                    self.advance();
                    let right = self.parse_expression(self.rbp(op.clone()))?;
                    left = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, String> {
        if self.is_at_end() {
            return Err("Unexpected end of input".to_string());
        }

        match self.current().unwrap() {
            Token::FloatLiteral(n) => {
                self.advance();
                Ok(Expr::Number(NumericValue::Double(n)))
            }
            Token::IntLiteral(n) => {
                self.advance();
                Ok(Expr::Number(NumericValue::Long(n)))
            }
            Token::Identifier(label) => {
                self.advance();
                Ok(Expr::Variable(label.clone()))
            }
            Token::Operator(Operator::Minus) => {
                self.advance();
                let expr = self.parse_expression(100)?;
                Ok(Expr::BinaryOp {
                    op: Token::Operator(Operator::Minus),
                    left: Box::new(Expr::Number(NumericValue::Double(0.0))),
                    right: Box::new(expr),
                })
            }
            Token::Delimiter(Delimiter::LeftParen) => {
                self.advance();
                let expr = self.parse_expression(0)?;
                if self.current() != Some(Token::Delimiter(Delimiter::RightParen)) {
                    return Err("Expected ')'".to_string());
                }
                self.advance();
                Ok(expr)
            }
            token => Err(format!("Unexpected token in nud: {:?}", token)),
        }
    }

    fn lbp(&self, token: Token) -> Option<u8> {
        match token {
            Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => Some(10),
            Token::Operator(Operator::PlusAssign) | Token::Operator(Operator::MinusAssign) => Some(10),
            Token::Operator(Operator::Multiply) | Token::Operator(Operator::Divide) | Token::Operator(Operator::Modulo) => Some(20),
            Token::Operator(Operator::MultiplyAssign) | Token::Operator(Operator::DivideAssign) | Token::Operator(Operator::ModuloAssign) => Some(20),
            Token::Operator(Operator::Equal) | Token::Operator(Operator::NotEqual)=> Some(30),
            Token::Operator(Operator::Greater) | Token::Operator(Operator::GreaterEqual) => Some(30),
            Token::Operator(Operator::Less) | Token::Operator(Operator::LessEqual) => Some(30),
            Token::Delimiter(Delimiter::RightParen) => None,
            Token::FloatLiteral(_) | Token::Delimiter(Delimiter::LeftParen) | _ => None,
        }
    }

    fn rbp(&self, token: Token) -> u8 {
        match token {
            Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => 10,
            Token::Operator(Operator::PlusAssign) | Token::Operator(Operator::MinusAssign) => 10,
            Token::Operator(Operator::Multiply) | Token::Operator(Operator::Divide) | Token::Operator(Operator::Modulo) => 20,
            Token::Operator(Operator::MultiplyAssign) | Token::Operator(Operator::DivideAssign) | Token::Operator(Operator::ModuloAssign) => 20,
            Token::Operator(Operator::Equal) | Token::Operator(Operator::NotEqual) => 30,
            Token::Operator(Operator::Greater) | Token::Operator(Operator::GreaterEqual) => 30,
            Token::Operator(Operator::Less) | Token::Operator(Operator::LessEqual) => 30,
            _ => 0,
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Expr::Number(n) => write!(f, "{}", *n)?,
            Expr::Number(n) => write!(f, "{}", n.to_string())?,
            Expr::Variable(v) => write!(f, "{v:?}")?,
            Expr::BinaryOp { op, left, right } => {
                write!(f, "{}", *left)?;
                match op.clone() {
                    Token::Operator(Operator::Plus) => write!(f, " + ")?,
                    Token::Operator(Operator::Minus) => write!(f, " - ")?,
                    Token::Operator(Operator::Multiply) => write!(f, " * ")?,
                    Token::Operator(Operator::Modulo) => write!(f, " % ")?,
                    Token::Operator(Operator::Divide) => write!(f, " / ")?,
                    Token::Operator(Operator::Equal) => write!(f, " == ")?,
                    Token::Operator(Operator::NotEqual) => write!(f, " != ")?,
                    _ => panic!("Unreachable branch. Invalid binary operator: {:?}", op),
                }
                write!(f, "{}", *right)?;
            }
        }
        Ok(())
    }
}

//todo: write tests
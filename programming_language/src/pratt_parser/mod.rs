// mod tokens;
// use tokens::*;
use crate::{pratt_parser::primitive_impl::PrimitiveNumber, tokens::{Delimiter, Keyword, Operator, Token}};
use core::num;
use std::fmt::{self, format};

mod primitive_impl;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl NumericValue {
    pub fn to_f64(&self) -> f64 {
        match self {
            NumericValue::Char(v) => *v as f64,
            NumericValue::UChar(v) => *v as f64,
            NumericValue::Short(v) => *v as f64,
            NumericValue::UShort(v) => *v as f64,
            NumericValue::Int(v) => *v as f64,
            NumericValue::UInt(v) => *v as f64,
            NumericValue::Long(v) => *v as f64,
            NumericValue::ULong(v) => *v as f64,
            NumericValue::Float(v) => *v as f64,
            NumericValue::Double(v) => *v,
        }
    }
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
    Label(String),
    FunctionCall(String, Vec<Expr>), //function name and called variables
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn coerce_to(&mut self, target: &NumericValue) {
        match self {
            Expr::Number(n) => {
                let new_val = match (*n, target) {
                    // signed conversions
                    (NumericValue::Char(v), NumericValue::Char(_)) => NumericValue::Char(v),
                    (NumericValue::Char(v), NumericValue::Short(_)) => NumericValue::Short(v as i16),
                    (NumericValue::Char(v), NumericValue::Int(_)) => NumericValue::Int(v as i32),
                    (NumericValue::Char(v), NumericValue::Long(_)) => NumericValue::Long(v as i64),
                    (NumericValue::Char(v), NumericValue::Float(_)) => NumericValue::Float(v as f32),
                    (NumericValue::Char(v), NumericValue::Double(_)) => NumericValue::Double(v as f64),

                    // unsigned conversions
                    (NumericValue::UChar(v), NumericValue::UChar(_)) => NumericValue::UChar(v),
                    (NumericValue::UChar(v), NumericValue::UShort(_)) => NumericValue::UShort(v as u16),
                    (NumericValue::UChar(v), NumericValue::UInt(_)) => NumericValue::UInt(v as u32),
                    (NumericValue::UChar(v), NumericValue::ULong(_)) => NumericValue::ULong(v as u64),
                    (NumericValue::UChar(v), NumericValue::Float(_)) => NumericValue::Float(v as f32),
                    (NumericValue::UChar(v), NumericValue::Double(_)) => NumericValue::Double(v as f64),

                    // general case: convert everything through f64 if no direct mapping
                    (val, NumericValue::Float(_)) => NumericValue::Float(val.to_f64() as f32),
                    (val, NumericValue::Double(_)) => NumericValue::Double(val.to_f64()),
                    (val, NumericValue::Int(_)) => NumericValue::Int(val.to_f64() as i32),
                    (val, NumericValue::UInt(_)) => NumericValue::UInt(val.to_f64() as u32),
                    (val, NumericValue::Long(_)) => NumericValue::Long(val.to_f64() as i64),
                    (val, NumericValue::ULong(_)) => NumericValue::ULong(val.to_f64() as u64),
                    (val, NumericValue::Short(_)) => NumericValue::Short(val.to_f64() as i16),
                    (val, NumericValue::UShort(_)) => NumericValue::UShort(val.to_f64() as u16),
                    (val, NumericValue::Char(_)) => NumericValue::Char(val.to_f64() as i8),
                    (val, NumericValue::UChar(_)) => NumericValue::UChar(val.to_f64() as u8),
                };
                *n = new_val;
            }
            Expr::FunctionCall(_, exprs) => {
                for e in exprs.iter_mut() {
                    e.coerce_to(target);
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                left.coerce_to(target);
                right.coerce_to(target);
            }
            Expr::Label(_) => {
                // labels probably represent variables — coercion
                // might be handled at type-check time, so no-op here
            }
        }
    }

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
                Ok(Expr::Label(label.clone()))
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
            Expr::Label(v) => write!(f, "{v:?}")?,
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
            },
            Expr::FunctionCall(a, b) => panic!("todo: implement string display for function calls")
        }
        Ok(())
    }
}

//todo: write tests
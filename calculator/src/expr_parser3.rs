use core::panic;
use std::fmt;
use crate::tokens::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,

    None(f64), //then it will be an exact number
}

impl OperationKind {
    pub fn weight(&self) -> u8 {
        match self {
            Self::None(_) => return 0,
            Self::Add | Self::Subtract => return 1,
            Self::Multiply | Self::Divide => return 2,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BinaryOp {
    pub has_prio: bool, //priority = if it has parentheses next to it, (only for printing)
    pub left_value: Option<Box<BinaryOp>>,
    pub right_value: Option<Box<BinaryOp>>,
    pub op_type: OperationKind,
}


pub struct BinaryOpParser {
    pub tokens: Vec<Token>,
    // b_tree: Option<BinaryOp>,
    pub idx: usize,
}

impl BinaryOpParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        BinaryOpParser { tokens, idx: 0 }
    }

    pub fn get_token(&self) -> Option<Token> {
        if self.tokens.len() <= self.idx {
            return None;
        }
        else {
            return Some(self.tokens[self.idx])
        }
        
    }

    pub fn next(&mut self){
        self.idx += 1;
    }

    pub fn check_back(&mut self, check_against: Token) -> bool{
        if self.tokens.len() < self.idx - 1 {
            return self.tokens[self.idx-1] == check_against;
        }
        false
    }

    pub fn progress(&mut self, amount: usize) {
        self.idx += amount;
    }
}

//only use when certain that the token is an operator
fn match_op_kind(op_token: Token) -> OperationKind {
    match op_token {
        Token::Add => OperationKind::Add,
        Token::Subtract => OperationKind::Subtract,
        Token::Multiply => OperationKind::Multiply,
        Token::Divide => OperationKind::Divide,
        o => panic!("invalid operation {o:?}")
    }
}
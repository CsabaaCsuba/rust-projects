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

impl BinaryOp {
    pub fn new(has_prio: bool, left_val: Option<BinaryOp>, right_val: Option<BinaryOp>, op_type: OperationKind) -> Self {
        let left_value = match left_val {
            Some(b_op) => Some(Box::new(b_op)),
            _ => None
        };
        let right_value = match right_val {
            Some(b_op) => Some(Box::new(b_op)),
            _ => None
        };
        return BinaryOp {has_prio, left_value, right_value, op_type}
    }
    pub fn number(num: f64) -> Self {
        return BinaryOp { has_prio: false, left_value: None, right_value: None, op_type: OperationKind::None(num) }
    }
    pub fn numbers_op(num1: f64, num2: f64, operation: OperationKind) -> Self {
        match operation {
            OperationKind::None(_) => panic!("operationkind cannot be none while also having the 2 operands."),
            _ => return BinaryOp{has_prio: false, left_value: Some(Box::new(BinaryOp::number(num1))), right_value: Some(Box::new(BinaryOp::number(num2))), op_type: operation},
        }
    }


    pub fn eval(&self) -> f64 {
        match self.op_type {
            OperationKind::None(num) => num,
            real_operation => {
                let left_op = *self.left_value.clone().unwrap();
                let left_num = left_op.eval();
                let right_op = *self.right_value.clone().unwrap();
                let right_num = right_op.eval();

                match real_operation {
                    OperationKind::Add => left_num + right_num,
                    OperationKind::Subtract => left_num - right_num,
                    OperationKind::Multiply => left_num * right_num,
                    OperationKind::Divide => left_num / right_num,
                    _ => panic!("impossible operation found")
                }
            }
        }

    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_prio {
            write!(f, "(")?;
        }

        match &self.op_type {
            OperationKind::None(n) => write!(f, "{n}")?,
            kind => {
                if let Some(left) = &self.left_value {
                    write!(f, "{left}")?;
                }

                let op_str = match kind {
                    OperationKind::Add => "+",
                    OperationKind::Subtract => "-",
                    OperationKind::Multiply => "*",
                    OperationKind::Divide => "/",
                    _ => panic!("invalid operation type between numbers"),
                };

                write!(f, "{op_str}")?;

                if let Some(right) = &self.right_value {
                    write!(f, "{right}")?;
                }
                if self.has_prio {
                    write!(f, ")")?;
                }
            }
        }

        

        Ok(())
    }
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


// pub fn parse_expr(parser: &mut BinaryOpParser) -> BinaryOp {
//     let mut return_op = BinaryOp::number(0.0);
//     match parser.get_token(){
//         Token::Number(n) => return_op.op_type = OperationKind::None(n),
//         Token::OpenParenthesis => {
//             return_op.has_prio = true;
//             parser.next();
//             match parser.get_token() {
//                 Token::Number(n)
//             }
//         },

//     }

//     return_op
// }

// pub fn parse_expr_nonum(parser: &mut BinaryOpParser, has_prio: bool) -> BinaryOp {

// }
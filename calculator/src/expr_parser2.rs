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

pub fn parse_expr(parser: &mut BinaryOpParser) -> BinaryOp {
    let mut parsed_left_number = 0.0;
    let mut return_op = BinaryOp::new(false, None, None, OperationKind::None(0.0));
    //expected: - sign, or a number, or (
    match parser.get_token() {
        Some(Token::Number(n)) => {return_op.op_type = OperationKind::None(n); parsed_left_number = n},
        Some(Token::Subtract) => {
            parser.next();
            match parser.get_token() {
                Some(Token::Number(n)) => {return_op.op_type = OperationKind::None(-n); parsed_left_number = -n},
                _ => panic!("Expected number after negative sign")
            }
        },
        Some(Token::OpenParenthesis) => {
            //this will have operational precedence anyway
            parser.next();
            let mut inner_expr = parse_expr(parser);
            inner_expr.has_prio = true;
            return_op.left_value = Some(Box::new(inner_expr));
            parser.next();
            match parser.get_token(){
                Some(Token::ClosedParenthesis) => (),
                _ => panic!("Closed parenthesis not found in it's place")
            }
        }
        Some(t) => panic!("Invalid token: {t:?}"),
        None => panic!("No token found at all")
    }

    parser.next();

    //expected: +, -, *, /, or nothing
    match parser.get_token() {
        Some(Token::Number(n)) => panic!("Number \"{n}\" after an other number is invalid"),
        Some(op) => { //operator was found, the previously parsed expression will be on the left of the current expression
            return_op = BinaryOp::new(false, Some(return_op), None, match_op_kind(op));
        },
        None => {
            match return_op.left_value {
                Some(val) => return *val, //since a parenthesis value was found, just make it the priority one, and return it
                None => return BinaryOp::number(parsed_left_number)//only a number was found, return
            }
            
        }
    }

    parser.next();
    //todo: also, if the expression ends in something like +9, that has to handled too

    //expected: any other expression, be it number, parenthesis, or anything else
    let mut parse_result = parse_expr(parser);
    match parse_result.op_type {
        OperationKind::None(_) => return_op.right_value = Some(Box::new(parse_result)),
        
        _ => {
            //this might not be good yet
            //check if found something with a larger operational precedence
            if (!parse_result.has_prio && !return_op.has_prio) && (parse_result.op_type.weight() >= return_op.op_type.weight()){
                //because operations go from left to right
                return_op.right_value = Some(Box::new(parse_result));
            }
            else {
                let aux = parse_result.clone();

                parse_result.op_type = return_op.op_type;
                return_op.op_type = aux.op_type;
                
                parse_result.right_value = return_op.right_value;
                
                
                parse_result.left_value = aux.right_value;
                return_op.right_value = Some(Box::new(parse_result));
                return_op.left_value = aux.left_value;

            }
        }
    }

    return_op
}


use core::{fmt, panic};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    OpenParenthesis,
    ClosedParenthesis,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,

    None(f64), //then it will be an exact number
}

impl OperationKind {
    fn weight(&self) -> u8 {
        match self {
            Self::None(_) => return 0,
            Self::Add | Self::Subtract => return 1,
            Self::Multiply | Self::Divide => return 2,
        }
    }
}



#[derive(Debug, Clone)]
pub(crate) struct BinaryOp {
    has_prio: bool,                     //priority = if it has parentheses next to it
    left_value: Option<Box<BinaryOp>>,
    right_value: Option<Box<BinaryOp>>,
    op_type: OperationKind,
}

impl BinaryOp {
    fn new(has_prio: bool, left_val: Option<BinaryOp>, right_val: Option<BinaryOp>, op_type: OperationKind) -> Self {
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
    fn number(num: f64) -> Self {
        return BinaryOp { has_prio: false, left_value: None, right_value: None, op_type: OperationKind::None(num) }
    }
    fn numbers_op(num1: f64, num2: f64, operation: OperationKind) -> Self {
        match operation {
            OperationKind::None(_) => panic!("operationkind cannot be none while also having the 2 operands."),
            _ => return BinaryOp{has_prio: false, left_value: Some(Box::new(BinaryOp::number(num1))), right_value: Some(Box::new(BinaryOp::number(num2))), op_type: operation},
        }
    }
    fn stringify(&self) -> String {
        return format!("{{{}, {:#?}, {:#?}, {:?}}}", self.has_prio, self.left_value, self.right_value, self.op_type);
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let OperationKind::None(num) = self.op_type {
            write!(f, "NoOp{{{}}}", num)?
        }
        match &self.right_value {
            Some(right_val) => {
                match &self.left_value {
                    Some(left_val) => {
                        return write!(f, "{{{}, {}, {}, {:?}}}", self.has_prio, left_val.stringify(), right_val.stringify(), self.op_type)
                    },
                    None => {write!(f, "No left value found..")}
                }

            },
            None => {write!(f, "No right value found..")}
        }
        
    }
}


fn check_binary_fit(tokens_slice: &[Token]) -> Option<BinaryOp>{
    let num1: f64;
    let num2: f64;
    let operator: OperationKind;

    match tokens_slice[0]{
        Token::Number(n) => num1 = n,
        _ => return None
    }
    match tokens_slice[1]{
        Token::Add => operator = OperationKind::Add,
        Token::Subtract => operator = OperationKind::Subtract,
        Token::Multiply => operator = OperationKind::Multiply,
        Token::Divide => operator = OperationKind::Divide,
        _ => return None
    }
    match tokens_slice[2]{
        Token::Number(n) => num2 = n,
        _ => return None
    }
    return Some(BinaryOp::numbers_op(num1, num2, operator));
}

fn calculate_op_value(expr: &BinaryOp) -> usize {
    let mut val: usize =  0;
    if expr.has_prio {
        val += 100;
    }
    if expr.op_type == OperationKind::Multiply || expr.op_type == OperationKind::Divide {
        val += 10;
    }
    if expr.op_type == OperationKind::Add || expr.op_type == OperationKind::Subtract {
        val += 1;
    }

    return val;
}

fn check_pattern_fit(checker: &[Token], checked_against: &[Token]) -> bool {
    assert_eq!(checker.len(), checked_against.len());
    for (idx, t) in checker.iter().enumerate(){
        if *t != checked_against[idx] {
            return false;
        }
    }
    true
}

unsafe fn parse_expression(expr_tokens: &Vec<Token>, idx: *mut usize) -> Option<BinaryOp> {
    println!("->recursive call started");
    let mut i = *idx;
    let tokens_len = expr_tokens.len();
    //todo: make this an option and refactor writing to this variable every time
    let mut return_op: BinaryOp = BinaryOp { has_prio: false, left_value: None, right_value: None, op_type: OperationKind::None(0.0) };
    let mut op_found = false;
    let has_prio: bool = match expr_tokens[i] {
        Token::OpenParenthesis => {
            println!("Priority open parenthesis found.");
            i += 1;
            true
        },
        _ => false,
    };
    
    //todo: refactor this to a be a function with non-reduntant code
    if i == tokens_len-1 || expr_tokens.get(i+1) == Some(&Token::ClosedParenthesis) {
        println!("Current pattern: num");
        if let Token::Number(num) = expr_tokens[i]{
            i += 1;
            return_op = BinaryOp::number(num);
            op_found = true;
        }
        else {
            panic!("The last token ({:?}) does not adhere to any rules", expr_tokens[i]);   
        }
    }
    // if let Some(Token::ClosedParenthesis) = expr_tokens.get(i+1) {
    //     println!("Current pattern: num");
    //     if let Token::Number(num) = expr_tokens[i]{
    //         i += 1;
    //         return_op = BinaryOp::number(num);
    //         op_found = true;
    //     }
    //     else {
    //         panic!("The last token ({:?}) does not adhere to any rules", expr_tokens[i]);   
    //     }
    // }
    //checks if the next operation is a simple binary operation
    else if i+2 <= tokens_len && !op_found {
        println!("Current pattern: num + num");
        if let Some(binary_op) = check_binary_fit(&expr_tokens[i..i+3]) {
            i += 3;
            return_op = binary_op;
            op_found = true;
        }
        //this will check for patterns like 3 + (2 + 10)
        //TODO: test this
        else if let Token::Number(n) = expr_tokens[i] {
            match expr_tokens[i+1] {
                Token::Add | Token::Subtract | Token::Multiply | Token::Divide => {
                    let operation_idx = i + 1;
                    match expr_tokens[i+2] {
                        Token::OpenParenthesis => {
                            println!("trying pattern num + (expression)");
                            i += 3;
                            *idx = i;
                            let expr = parse_expression(expr_tokens, idx);
                            i = *idx;
                            if let Some(new_expr) = expr {
                                let op_type = match expr_tokens[operation_idx] {
                                    Token::Add => OperationKind::Add,
                                    Token::Subtract => OperationKind::Subtract,
                                    Token::Multiply => OperationKind::Multiply,
                                    Token::Divide => OperationKind::Divide,
                                    _ => panic!("other operations are impossible")
                                };
                                println!("Pattern num + (expression) found.");
                                return_op = BinaryOp::new(false, Some(BinaryOp::number(n)), Some(new_expr), op_type);
                                if let Some(Token::ClosedParenthesis) = expr_tokens.get(i) {
                                    op_found = true;
                                    if let Some(expr) = &mut return_op.left_value {
                                        expr.has_prio = true;
                                    }
                                    // i += 1;
                                    
                                }
                                else {
                                    panic!("num + (expression) block didn't end with a closed parenthesis. Instead it ended with {:#?}", expr_tokens.get(i+1));
                                }
                            }
                        },
                        _ => panic!("invalid token ({:?}) at index {}", expr_tokens[i+2], i+2)
                    }
                },
                _ => (),
            }
        }
        else {
            println!("pattern num + num not satisfied ({:?})", &expr_tokens[i..i+3]);
            // panic!("test panic for return_op");
        }
        

    }

    if !op_found {
    if let Some(Token::Number(n)) = expr_tokens.get(i) {
        //TODO: create matcher for patterns like 3 + (4 * 2) [use recursive calls]
        i += 1;
        match expr_tokens[i] { //TODO: use .get without unwrapping
            Token::Add | Token::Subtract | Token::Multiply | Token::Divide => {
                let op_type = match expr_tokens[i]{
                    Token::Add => OperationKind::Add,
                    Token::Subtract => OperationKind::Subtract,
                    Token::Multiply => OperationKind::Multiply,
                    Token::Divide => OperationKind::Divide,
                    _ => panic!("other operations are impossible")
                };

                println!("Current pattern: num + expr");

                i += 1;
                *idx = i;
                return_op = BinaryOp{has_prio, left_value: Some(Box::new(BinaryOp::number(*n))), right_value:None, op_type};
                let right_value = parse_expression(expr_tokens, idx);
                // *idx+=2;
                println!("=====got back from expression parsing.");
                match right_value {
                    Some(operation) => {
                        println!("Operation of {:?} found.", operation.op_type);
                        return_op.right_value = Some(Box::new(operation));
                        op_found = true;
                        i = *idx;
                    }
                    None => {
                        panic!("Expression expected as the continuation of parsing.");
                    }
                }
            },
            
            other_token => {
                println!("num + expr not satisfied (token {:?}", other_token);
                return None;
                // panic!("Unexpected token after number '{}': {:?}", n, other_token);
                // i -= 1;
            }
        }
    }
    }

    if has_prio {
        if expr_tokens[i] == Token::ClosedParenthesis {
            println!(">>>>>>>>>>Found expression has priority (parentheses) after it.");
            return_op.has_prio = true;
            i += 1;
        }
        else {
            println!(">>>>>>>>>>Found expression has no priority.");
        }  
    }

    //if something has been parsed before, it can have something like * 7
    //syntax like 6 + 3 (* 7) isn't allowed of course
    //this should be checked if an operation has been found
    if i <= tokens_len-2 && op_found { // && !has_prio //-3 because there should be 2 tokens at least
        println!("checking possibility of a parsed_expr + expr pattern...");
        match expr_tokens.get(i){
            Some(Token::Number(num)) => panic!("Number ({num:?}) is not valid here."),
            Some(Token::OpenParenthesis) | Some(Token::ClosedParenthesis) => panic!("Parentheses are not valid here"),
            Some(op_token) => {
                let op_type = match op_token{
                    Token::Add => OperationKind::Add,
                    Token::Subtract => OperationKind::Subtract,
                    Token::Multiply => OperationKind::Multiply,
                    Token::Divide => OperationKind::Divide,
                    _ => panic!("other operations are impossible")
                };
                i += 1;
                *idx = i;
                let other_wing = parse_expression(expr_tokens, idx);
                match other_wing {
                    Some(mut new_expr) => {
                        //if the newly parsed expression is just a number, we don't have to care about it's op-type
                        if let OperationKind::None(num) = new_expr.op_type {
                            //if the op_type is multiply or divide, it should be below the current 
                            if op_type == OperationKind::Multiply || op_type == OperationKind::Divide {
                                let saved_op = return_op.right_value;
                                new_expr.left_value = saved_op;
                                new_expr.op_type = op_type;
                                new_expr.right_value = Some(Box::new(BinaryOp::number(num)));
                                return_op.right_value = Some(Box::new(new_expr));
                            }
                            else if op_type == OperationKind::Add || op_type == OperationKind::Subtract {
                                new_expr.op_type = return_op.op_type;
                                new_expr.left_value = return_op.left_value;
                                new_expr.right_value = return_op.right_value;
                                return_op.op_type = op_type;
                                return_op.right_value = Some(Box::new(BinaryOp::number(num)));
                                return_op.left_value = Some(Box::new(new_expr));
                            }
                            else {
                                panic!("Outer op_type matcher should have caught this error.");
                            }
                        }
                        else {
                            //todo: make a plan on implementing this as well.

                            //if the expressions are on the same level of operation (either [*, /] or [+, -]), and neither one has parentheses
                            if return_op.op_type.weight() == new_expr.op_type.weight() && return_op.op_type.weight() == op_type.weight()
                                && return_op.has_prio == false && new_expr.has_prio == false {
                                
                                let new_expr_left = new_expr.left_value.unwrap();
                                new_expr.left_value = Some(Box::new(BinaryOp::new(false, Some(return_op.clone()), Some(*new_expr_left), op_type)));
                                return_op = new_expr;
                            }
                        }
                        

                        
                        i = *idx;
                    },
                    None => panic!("No valid expresison found after {op_token:?}")
                }
            },
            other_val => panic!("unexpected value when trying to parse right-side value of binary expression: {other_val:?}"),
        }
    }

    
    
    if !op_found {
        println!("unexpected token: {:?}", expr_tokens.get(i));
        return None;
    }

    if has_prio && !return_op.has_prio {
        if expr_tokens[i] == Token::ClosedParenthesis {
            println!("Closing parenthesis has been found on the last check.");
            return_op.has_prio = true;
            i += 1;
        }
        else {
            panic!("Closing parenthesis wasn't found...");
        }  
    }

    
    
    *idx = i; //this might cause bugs since recursive calls will set *idx forward

    return Some(return_op);
}

fn main() {
    // let expression = "5 + (3*2)";
    let expression_tokens = vec![
        Token::Number(5.0),
        Token::Add,
        Token::OpenParenthesis,
        Token::Number(3.0),
        Token::Multiply,
        Token::Number(2.0),
        Token::ClosedParenthesis,
    ];

    // let expression = "1 + 6 * 7";
    let expression_tokens_2 = vec![
        Token::Number(1.0),
        Token::Add,
        Token::Number(6.0),
        Token::Multiply,
        Token::Number(7.0),
        // Token::Divide,
        // Token::Number(2.0),
    ];

    //let expression = "3 - 2 + 4";
    let expression_tokens_3 = vec![
        Token::Number(3.0),
        Token::Subtract,
        Token::Number(2.0),
        Token::Add,
        Token::Number(4.0),
    ];

    //let expression = "9 + 8 - 3 + 4";
    let expression_tokens_4 = vec![
        Token::Number(9.0),
        Token::Add,
        Token::Number(8.0),
        Token::Subtract,
        Token::Number(3.0),
        Token::Add,
        Token::Number(4.0),
    ];

    //let expression = "9 + (8 - 3)";
    let expression_tokens_5 = vec![
        Token::Number(9.0),
        Token::Add,
        Token::OpenParenthesis,
        Token::Number(8.0),
        Token::Subtract,
        Token::Number(3.0),
        Token::ClosedParenthesis,
    ];

    //let expression = "5 + 2 * 3 - 4";
    // let expression_tokens_6 = vec![
    //     Token::Number(5.0),
    //     Token::Add,
    //     Token::Number(2.0),
    //     Token::Multiply,
    //     Token::Number(3.0),
    //     Token::Subtract,
    //     Token::Number(4.0),
    // ];

    let mut i: usize = 0;
    unsafe {
        // let parsed_expr = parse_expression(&expression_tokens, &mut i);
        // println!("parsed expression: {parsed_expr:?}");
        // let parsed_expr_2 = parse_expression(&expression_tokens_2, &mut i);
        // println!("parsed expression: {parsed_expr_2:?}");
        i = 0;
        let parsed_expr_5 = parse_expression(&expression_tokens_5, &mut i).unwrap();
        println!("parsed expression: {parsed_expr_5:?}");
    }
    

}

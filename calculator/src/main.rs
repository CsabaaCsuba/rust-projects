use core::panic;

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



#[derive(Debug, Clone)]
pub(crate) struct BinaryOp {
    has_prio: bool,                     //priority = if it has parentheses next to it
    left_value: Option<Box<BinaryOp>>,
    right_value: Option<Box<BinaryOp>>,
    op_type: OperationKind,
}

impl BinaryOp {
    fn number(num: f64) -> Self {
        return BinaryOp { has_prio: false, left_value: None, right_value: None, op_type: OperationKind::None(num) }
    }
    fn numbers_op(num1: f64, num2: f64, operation: OperationKind) -> Self {
        match operation {
            OperationKind::None(_) => panic!("operationkind cannot be none while also having the 2 operands."),
            _ => return BinaryOp{has_prio: false, left_value: Some(Box::new(BinaryOp::number(num1))), right_value: Some(Box::new(BinaryOp::number(num2))), op_type: operation},
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

unsafe fn parse_expression(expr_tokens: &Vec<Token>, idx: *mut usize) -> Option<BinaryOp> {
    println!("->recursive call started");
    let mut i = *idx;
    let tokens_len = expr_tokens.len();
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

    //only the last token can be a number on it's own + a token inside parentheses
    let minus_idx: usize = match has_prio {
        true => 2,
        false => 1,
    };
    
    if i == tokens_len-minus_idx {
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
    //checks if the next operation is a simple binary operation
    else if i+2 <= tokens_len && !op_found {
        println!("Current pattern: num + num");
        if let Some(binary_op) = check_binary_fit(&expr_tokens[i..i+3]) {
            i += 3;
            return_op = binary_op;
            op_found = true;
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

    
    
    if !op_found {
        println!("unexpected token: {:?}", expr_tokens.get(i));
        return None;
    }

    if has_prio {
        if expr_tokens[i] != Token::ClosedParenthesis {
            println!("Expression Started with parentheses, expects parentheses at the end");
            return None;
        }
        return_op.has_prio = true;
    }

    
    
    *idx = i;

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

    let mut i: usize = 0;
    unsafe {
        let parsed_expr = parse_expression(&expression_tokens, &mut i);
        println!("parsed expression: {parsed_expr:?}");
    }
    

}

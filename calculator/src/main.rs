#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    AddSymbol,
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
    let mut i = *idx;
    let tokens_len = expr_tokens.len();
    let return_op: BinaryOp;
    let mut has_prio: bool = match expr_tokens[i] {
        Token::OpenParenthesis => {
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
        if let Token::Number(num) = expr_tokens[i]{
            i += 1;
            return_op = BinaryOp::number(num);
        }
        else {
            println!("The last token ({:?}) does not adhere to any rules", expr_tokens[i]);
            return None;
        }
        
    }

    //checks if the next operation is a simple binary operation
    else if i+2 <= tokens_len {
        if let Some(binary_op) = check_binary_fit(&expr_tokens[i..i+3]) {
            i += 3;
            return_op = binary_op;
        }
    }
    else {
        //TODO: create matcher for patterns like 3 + (4 * 2) [use recursive calls]
        match expr_tokens[i]{

        }
    }

    if has_prio {
        if expr_tokens[i] != Token::ClosedParenthesis {
            println!("Expression Started with parentheses, expects parentheses at the end");
            return None;
        }
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
    }
    

}

use std::iter::Peekable;
use std::collections::HashMap;
use crate::{Token::*, lexer::{Lex}, Token, Ops::*, Ops};
use mut_static::MutStatic;
use thiserror::Error;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref OP_PRECEDENCE: MutStatic<HashMap<Ops, i32>> = {
        let mut map = HashMap::new();
        map.insert(Ops::Assign, 2);
        map.insert(Ops::Plus, 20);
        map.insert(Ops::Minus, 20);
        map.insert(Ops::Mult, 40);
        map.insert(Ops::Div, 40);
        map.insert(Ops::Eq, 50);
        map.insert(Ops::Neq, 50);
        map.insert(Ops::Gt, 50);
        map.insert(Ops::Lt, 50);
        map.into()
    };
}

// Enum dispatch instead?
// - less indirection trait objects and Vtables, faster
// - keep string references, no copies
// - No trait objects so I can easily derive PartialEq
#[derive(Debug, Clone, PartialEq)]
pub enum ASTExpr<'src> {
    NumberExpr(f64),
    VariableExpr(&'src str),
    UnaryExpr {
        op: Ops,
        operand: Box<ASTExpr<'src>>,
    },
    BinaryExpr {
        op: Ops,
        left: Box<ASTExpr<'src>>,
        right: Box<ASTExpr<'src>>,
    },
    CallExpr {
        callee: &'src str,
        args: Vec<Box<ASTExpr<'src>>>,
    },
    IfExpr {
        cond: Box<ASTExpr<'src>>,
        then_branch: Box<ASTExpr<'src>>,
        else_branch: Box<ASTExpr<'src>>,
    },
    ForLoopExpr {
        varname: &'src str,
        start: Box<ASTExpr<'src>>,
        end: Box<ASTExpr<'src>>,
        step: Box<ASTExpr<'src>>,
        body: Box<ASTExpr<'src>>,
    },
    VarExpr {
        var_names: Vec<(&'src str, Option<Box<ASTExpr<'src>>>)>, // var_names is a combination of variable name and (possible) initializer
        body: Box<ASTExpr<'src>>,
    },
}

// Prototype
#[derive(Debug, PartialEq)]
pub struct Prototype<'src> {
    pub name: &'src str,
    pub args: Vec<&'src str>,
}

// Function
#[derive(Debug, PartialEq)]
pub struct Function<'src> {
    pub proto: Box<Prototype<'src>>,
    pub body: Box<ASTExpr<'src>>,
}


#[derive(Error, PartialEq, Debug)]
pub enum ParserError<'src> {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token<'src>),

    #[error("Reached end of input expecting more")]
    UnexpectedEOI,

    #[error("Expected token: {0:?}")]
    ExpectedToken(Token<'src>),
}

type ParseResult<'src> = Result<Box<dyn AST>, ParserError<'src>>;

/// external ::= 'extern' prototype
pub fn parse_extern<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> Result<Box<Prototype<'src>>, ParserError<'src>> {
    // Swallow the 'extern' keyword, parse as prototype
    let _extern = tokens.next();
    parse_prototype(tokens)
}

/// prototype
///   ::= id '(' id* ')'
pub fn parse_prototype<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> Result<Box<Prototype<'src>>, ParserError<'src>> {
    match tokens.next() {
        Some(Token::Identifier(name)) => {
            let _ = tokens
                .next_if(|t| matches!(t, Token::OpenParen))
                .ok_or(ParserError::ExpectedToken(&"("))?;

            let mut args = vec![];

            while let Some(Token::Identifier(s)) = tokens.peek() {
                args.push(*s);
                let _ = tokens.next();
            }

            let _ = tokens
                .next_if(|t| matches!(t, Token::ClosedParen))
                .ok_or(ParserError::ExpectedToken(&")"))?;

            Ok(Box::new(Prototype::FunctionProto { name, args }))
        }

        Some(Token::UnaryOverload) => {
            let Some(Token::Operator(operator)) = tokens.next() else {
                return Err(ParserError::ExpectedToken("!/&/|/^/:"));
            };

            // swallow open parenthesis
            let _ = tokens
                .next_if(|t| matches!(t, Token::OpenParen))
                .ok_or(ParserError::ExpectedToken(&"("))?;

            let Some(Token::Identifier(arg)) = tokens.next() else {
                return Err(ParserError::BadOverloadedUnaryOp);
            };

            // swallow closed parenthesis
            let _ = tokens
                .next_if(|t| matches!(t, Token::ClosedParen))
                .ok_or(ParserError::ExpectedToken(&")"))?;

            Ok(Box::new(Prototype::OverloadedUnaryOpProto {
                operator,
                arg,
            }))
        }

        Some(Token::BinaryOverload) => {
            let Some(Token::Operator(operator)) = tokens.next() else {
                return Err(ParserError::ExpectedToken("!/&/|/^/:"));
            };

            let Some(Token::Number(precedence)) = tokens.next() else {
                return Err(ParserError::BadOverloadedBinaryOp);
            };

            let mut precedence_map = OP_PRECEDENCE.write().unwrap();

            precedence_map.insert(operator, precedence.ceil() as i32);

            // swallow open parenthesis
            let _ = tokens
                .next_if(|t| matches!(t, Token::OpenParen))
                .ok_or(ParserError::ExpectedToken(&"("))?;

            let (Some(Token::Identifier(lhs)), Some(Token::Identifier(rhs))) =
                (tokens.next(), tokens.next())
            else {
                return Err(ParserError::BadOverloadedUnaryOp);
            };

            // swallow closed parenthesis
            let _ = tokens
                .next_if(|t| matches!(t, Token::ClosedParen))
                .ok_or(ParserError::ExpectedToken(&")"))?;

            Ok(Box::new(Prototype::OverloadedBinaryOpProto {
                operator,
                precedence: precedence.ceil() as i32,
                args: (lhs, rhs),
            }))
        }

        Some(unexpected) => Err(ParserError::UnexpectedToken(unexpected)),
        None => Err(ParserError::UnexpectedEOI),
    }
}

/// definition ::= 'def' prototype expression
pub fn parse_definition<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> Result<Box<Function<'src>>, ParserError<'src>> {
    // swallow the def keyword
    let _def = tokens.next();

    // try to parse prototype and body
    let proto = parse_prototype(tokens)?;
    let body = parse_expression(tokens)?;

    Ok(Box::new(Function { proto, body }))
}

/// toplevelexpr ::= expression
pub fn parse_top_level_expr<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> Result<Box<Function<'src>>, ParserError<'src>> {
    let expr = parse_expression(tokens)?;

    let proto = Box::new(Prototype::FunctionProto {
        name: &"__anonymous_expr",
        args: vec![],
    });

    Ok(Box::new(Function { proto, body: expr }))
}

// Small alias for fallible returns of parsing expressions
type ExprParseResult<'src> = Result<Box<ASTExpr<'src>>, ParserError<'src>>;

/// primary
///   ::= identifierexpr
///   ::= numberexpr
///   ::= parenexpr
///   ::= ifexpr
///   ::= forloopexpr
///   ::= varexpr
fn parse_primary<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    match tokens.peek() {
        Some(Token::Identifier(_)) => parse_identifier_expr(tokens),

        Some(Token::Number(_)) => parse_number_expr(tokens),

        Some(Token::OpenParen) => parse_paren_expr(tokens),

        Some(Token::If) => parse_if_expr(tokens),

        Some(Token::For) => parse_for_loop_expression(tokens),

        Some(Token::Var) => parse_var_expression(tokens),

        Some(unexpected) => Err(ParserError::UnexpectedToken(*unexpected)),

        None => Err(ParserError::UnexpectedEOI),
    }
}

/// varexpr ::= 'var' identifier ('=' expression)?
//              (',' identifier ('=' expression)?)* 'in' expression
fn parse_var_expression<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    // Swallow the var keyword
    let _ = tokens.next();

    let mut var_names = vec![];

    // Loop over the list of comma delimited variables with possible initializers
    loop {
        let Some(Token::Identifier(name)) = tokens.next() else {
            return Err(ParserError::ExpectedToken("<identifier>"));
        };

        // If there is an assignment operator following, it has an initializer,
        // parse it and add it along with name, otherwise there is no initializer
        if let Some(Token::Operator(Ops::Assign)) = tokens.peek() {
            let _assign = tokens.next();
            let init = parse_expression(tokens)?;

            var_names.push((name, Some(init)));
        } else {
            var_names.push((name, None))
        }

        // If we have a comma following, we loop, otherwise, we break out of loop
        if let None = tokens.next_if(|t| matches!(t, Token::Comma)) {
            break;
        }
    }

    // Check for the "in" keyword, should be there before body
    tokens
        .next_if(|t| matches!(t, Token::In))
        .ok_or(ParserError::ExpectedToken(&"in"))?;

    let body = parse_expression(tokens)?;

    Ok(Box::new(ASTExpr::VarExpr { var_names, body }))
}

/// unary
///   ::= primary
///   ::= '!' unary
fn parse_unary<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    if let Some(Token::Operator(op)) = tokens.next_if(|t| matches!(t, Token::Operator(_))) {
        let operand = parse_unary(tokens)?;

        Ok(Box::new(ASTExpr::UnaryExpr { op, operand }))
    } else {
        parse_primary(tokens)
    }
}

/// forexpr ::= 'for' identifier '=' expression ',' expression (',' expr)? 'in' expression
fn parse_for_loop_expression<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    let Some(Token::For) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"for"));
    };

    let Some(Token::Identifier(varname)) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"variable"));
    };

    let Some(Token::Operator(Ops::Assign)) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"="));
    };

    let start = parse_expression(tokens)?;

    let Some(Token::Comma) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&","));
    };

    let end = parse_expression(tokens)?;

    // Step is optional in the loop, but the absence is understood to be an increment of 1.0 per loop iteration
    let step = {
        if let Some(Token::Comma) = tokens.next_if(|token| matches!(token, Token::Comma)) {
            parse_expression(tokens)?
        } else {
            Box::new(ASTExpr::NumberExpr(1.0))
        }
    };

    let Some(Token::In) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"in"));
    };

    let body = parse_expression(tokens)?;

    Ok(Box::new(ASTExpr::ForLoopExpr {
        varname,
        start,
        end,
        step,
        body,
    }))
}

/// ifexpr ::= 'if' expression 'then' expression 'else' expression
fn parse_if_expr<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    let Some(Token::If) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"if"));
    };

    let cond = parse_expression(tokens)?;

    let Some(Token::Then) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"then"));
    };

    let then_branch = parse_expression(tokens)?;

    let Some(Token::Else) = tokens.next() else {
        return Err(ParserError::ExpectedToken(&"else"));
    };

    let else_branch = parse_expression(tokens)?;

    Ok(Box::new(ASTExpr::IfExpr {
        cond,
        then_branch,
        else_branch,
    }))
}

/// numberexpr ::= number
fn parse_number_expr<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    if let Some(Token::Number(num)) = tokens.next() {
        Ok(Box::new(ASTExpr::NumberExpr(num)))
    } else {
        panic!("Expected next token to be number for parse_number_expr!")
    }
}

/// identifierexpr
///   ::= identifier
///   ::= identifier '(' expression* ')'
fn parse_identifier_expr<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => name,
        _unexpected => panic!("Expected"),
    };

    // Call Expression
    if let Some(Token::OpenParen) = tokens.peek() {
        let _open_paren = tokens.next();

        let mut args = vec![];

        loop {
            if let Some(Token::ClosedParen) = tokens.peek() {
                break;
            }

            parse_expression(tokens).map(|arg_expr| args.push(arg_expr))?;

            if let Some(Token::Comma) = tokens.peek() {
                tokens.next();
                continue;
            }
        }

        let _closed_paren = tokens.next();

        Ok(Box::new(ASTExpr::CallExpr { callee: name, args }))
    } else {
        // Variable Expression
        Ok(Box::new(ASTExpr::VariableExpr(name)))
    }
}

/// parenexpr ::= '(' expression ')'
fn parse_paren_expr<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    // Swallow the open parenthesis
    let _paren = tokens.next();

    // Parse the expression inside it
    let expr = parse_expression(tokens);

    // Should be a closed parenthesis following it.
    match tokens.next() {
        Some(Token::ClosedParen) => expr,
        Some(unexpected) => Err(ParserError::UnexpectedToken(unexpected)),
        None => Err(ParserError::UnexpectedEOI),
    }
}

/// expression
///   ::= primary binoprhs
///
fn parse_expression<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
) -> ExprParseResult<'src> {
    // Be sure we handle the case where either the lhs has unary
    // operator, or rhs, or both.
    let lhs = parse_unary(tokens)?;

    parse_binop_rhs(tokens, lhs, 0)
}

// Small helper method to fetch the precedence of operator
// from hash table. If the token is not an operator,
// default to -1. Tutorial names this GetTokPrecedence
fn get_token_precedence(token: Token) -> i32 {
    if let Token::Operator(operator) = token {
        OP_PRECEDENCE.read().unwrap()[&operator]
    } else {
        -1
    }
}

/// binoprhs
///   ::= ('+' primary)*
fn parse_binop_rhs<'src>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'src>>>,
    mut lhs: Box<ASTExpr<'src>>,
    expr_prec: i32,
) -> ExprParseResult<'src> {
    loop {
        let tok_prec = match tokens.peek().copied() {
            Some(token) => get_token_precedence(token),
            None => return Err(ParserError::UnexpectedEOI),
        };

        if tok_prec < expr_prec {
            return Ok(lhs);
        }

        let Some(Token::Operator(op)) = tokens.next() else {
            panic!("FATAL: misuse of of this function in recursive descent!")
        };

        // In chapter 6, we changed this from parse_primary to parse_unary
        // handle the lhs case where it might be attached to unary operator
        let mut rhs = parse_unary(tokens)?;

        let next_prec = match tokens.peek().copied() {
            Some(token) => get_token_precedence(token),
            None => return Err(ParserError::UnexpectedEOI),
        };

        if tok_prec < next_prec {
            rhs = parse_binop_rhs(tokens, rhs, tok_prec + 1)?;
        }

        lhs = Box::new(ASTExpr::BinaryExpr {
            op,
            left: lhs,
            right: rhs,
        })
    }
}
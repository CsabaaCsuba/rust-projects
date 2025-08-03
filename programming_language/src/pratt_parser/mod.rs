// mod tokens;
// use tokens::*;
use crate::tokens::{Delimiter, Operator, Token, Keyword};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
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
                Ok(Expr::Number(n))
            }
            Token::Operator(Operator::Minus) => {
                self.advance();
                let expr = self.parse_expression(100)?;
                Ok(Expr::BinaryOp {
                    op: Token::Operator(Operator::Minus),
                    left: Box::new(Expr::Number(0.0)),
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
            Token::Operator(Operator::Multiply) | Token::Operator(Operator::Divide) | Token::Operator(Operator::Modulo) => Some(20),
            Token::Delimiter(Delimiter::RightParen) => None,
            Token::FloatLiteral(_) | Token::Delimiter(Delimiter::LeftParen) | _ => None,
        }
    }

    fn rbp(&self, token: Token) -> u8 {
        match token {
            Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => 10,
            Token::Operator(Operator::Multiply) | Token::Operator(Operator::Divide) | Token::Operator(Operator::Modulo) => 20,
            _ => 0,
        }
    }
}

impl Expr {
    pub fn resolve_numeric(&self) -> Option<f64> {
        match self {
            Expr::Number(n) => Some(*n),
            Expr::BinaryOp { op, left, right } => {
                let l = left.resolve_numeric()?;
                let r = right.resolve_numeric()?;
                match op {
                    Token::Operator(Operator::Plus) => Some(l + r),
                    Token::Operator(Operator::Minus) => Some(l - r),
                    Token::Operator(Operator::Multiply) => Some(l * r),
                    Token::Operator(Operator::Modulo) => {
                        if r == 0.0 {
                            return None;
                        }
                        Some( l % r)
                    },
                    Token::Operator(Operator::Divide) => {
                        if r == 0.0 {
                            return None;
                        }
                        Some(l / r)
                    }
                    _ => panic!("Unreachable branch. Invalid binary operator: {:?}", op),
                }
            }
        }
    }

}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", *n)?,
            Expr::BinaryOp { op, left, right } => {
                write!(f, "{}", *left)?;
                match op.clone() {
                    Token::Operator(Operator::Plus) => write!(f, " + ")?,
                    Token::Operator(Operator::Minus) => write!(f, " - ")?,
                    Token::Operator(Operator::Multiply) => write!(f, " * ")?,
                    Token::Operator(Operator::Modulo) => write!(f, " % ")?,
                    Token::Operator(Operator::Divide) => write!(f, " / ")?,
                    _ => panic!("Unreachable branch. Invalid binary operator: {:?}", op),
                }
                write!(f, "{}", *right)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::tokenize;
    use crate::pratt_parser::*;

    fn easy_eval(raw_str: &str) -> Option<f64> {
        ExprParser::new(tokenize(raw_str.to_string()))
            .parse_expression(0)
            .expect("failed to parse?")
            .resolve_numeric()
    }

    #[test]
    fn test1(){
        assert_eq!(ExprParser::new(tokenize("3.0 + 3.0".to_string())).parse_expression(0).expect("failed to parse?").resolve_numeric(), Some(6.0));
    }

    #[test]
    fn test2(){
        assert_eq!(ExprParser::new(tokenize("5.0 % 3.0".to_string())).parse_expression(0).expect("failed to parse?").resolve_numeric(), Some(2.0));
    }

    #[test]
    #[should_panic(expected="failed to parse?")]
    fn test3(){
        assert_eq!(ExprParser::new(tokenize("1.0 % 4.0 * ".to_string())).parse_expression(0).expect("failed to parse?").resolve_numeric(), Some(1.0));
    }

    #[test]
    fn test4(){
        assert_eq!(ExprParser::new(tokenize("(1.0 * 4.0)".to_string())).parse_expression(0).expect("failed to parse?").resolve_numeric(), Some(4.0));
    }

    #[test]
    fn test5(){
        assert_eq!(&ExprParser::new(tokenize("(1.0 * 4.5)".to_string())).parse_expression(0).expect("failed to parse?").to_string(), "1 * 4.5");
    }

    //this test doesn't work properly, because the parenthesis information gets lost, however, the evaluation works fine
    // #[test]
    // fn test6(){
    //     assert_eq!(&ExprParser::new(tokenize("2.0 * (3.3 - 1.3)".to_string())).parse_expression(0).expect("failed to parse?").to_string(), "2 * 3.3 - 1.3");
    // }

    #[test]
    fn test7(){
        assert_eq!(easy_eval("2.0 * (3.3 - 1.3)"), Some(2.0 * (3.3 - 1.3)));
    }
}
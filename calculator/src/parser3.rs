use crate::tokens::Token;

#[derive(Debug)]
enum Expr {
    Number(f64),
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn parse_expression(&mut self, rbp: u8) -> Expr {
        let mut left = self.nud();

        while !self.is_at_end() {
            if let Some(current_token) = self.current() {
                if let Some(bp) = self.lbp(current_token) {
                    if bp <= rbp {
                        break;
                    }
                    
                    let op = current_token;
                    self.advance();
                    let right = self.parse_expression(self.rbp(op));
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

        left
    }

    fn nud(&mut self) -> Expr {
        if self.is_at_end() {
            panic!("Unexpected end of input");
        }

        match self.current().unwrap() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::Subtract => {
                self.advance();
                let expr = self.parse_expression(100);
                Expr::BinaryOp {
                    op: Token::Subtract,
                    left: Box::new(Expr::Number(0.0)),
                    right: Box::new(expr),
                }
            }
            Token::OpenParenthesis => {
                self.advance();
                let expr = self.parse_expression(0);
                if self.current() != Some(Token::ClosedParenthesis) {
                    panic!("Expected ')'");
                }
                self.advance();
                expr
            }
            token => panic!("Unexpected token in nud: {:?}", token),
        }
    }

    fn lbp(&self, token: Token) -> Option<u8> {
        match token {
            Token::Add | Token::Subtract => Some(10),
            Token::Multiply | Token::Divide => Some(20),
            Token::ClosedParenthesis => None,
            Token::Number(_) | Token::OpenParenthesis => None,
        }
    }

    fn rbp(&self, token: Token) -> u8 {
        match token {
            Token::Add | Token::Subtract => 10,
            Token::Multiply | Token::Divide => 20,
            _ => 0,
        }
    }
}

impl Expr {
    pub fn resolve(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::BinaryOp { op, left, right } => {
                let l = left.resolve();
                let r = right.resolve();
                match op {
                    Token::Add => l + r,
                    Token::Subtract => l - r,
                    Token::Multiply => l * r,
                    Token::Divide => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        l / r
                    }
                    _ => panic!("Invalid binary operator: {:?}", op),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::tokenize;

    fn eval(input: &str) -> f64 {
        let tokens = tokenize(input).unwrap();
        println!("Tokens: {:?}", tokens); // Debug print to see what tokens are produced
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression(0);
        expr.resolve()
    }

    #[test]
    fn test_addition() {
        assert_eq!(eval("1 + 2"), 3.0);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(eval("2 + 3 * 4"), 14.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval("(2 + 3) * 4"), 20.0);
    }

    #[test]
    fn test_nested() {
        assert_eq!(eval("1 + 2 * (3 - 4) + 5"), 4.0);
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(eval("-5 + 2"), -3.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        eval("4 / 0");
    }
}
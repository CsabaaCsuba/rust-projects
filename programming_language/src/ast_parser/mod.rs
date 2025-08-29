use crate::tokens::{Token, Keyword, Operator, Delimiter};
use crate::pratt_parser::{Expr, ExprParser, NumericValue};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Char,
    Short, 
    Int,
    Long,
    Float,
    Double,
    Unsigned(Box<Type>),
    Signed(Box<Type>),
    Pointer(Box<Type>),
    Array(Box<Type>, Option<i64>), // type, size (None for unsized)
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::Void => 0,
            Type::Char => 1,
            Type::Short => 2,
            Type::Int => 4,
            Type::Long => 8,
            Type::Float => 4,
            Type::Double => 8,
            Type::Unsigned(any) => any.clone().size(),
            Type::Signed(any) => any.clone().size(),
            Type::Pointer(_) => 8,
            Type::Array(_, size) => match size {
                Some(n) => *n as usize,
                None => panic!("Idk what to do with unsized arrays")
            },
        }
    }

    pub fn to_numeric_value(&self) -> Option<NumericValue> {
        match self {
            Type::Void => None,
            Type::Char => Some(NumericValue::Char(0)),
            Type::Short => Some(NumericValue::Short(0)),
            Type::Int => Some(NumericValue::Int(0)),
            Type::Long => Some(NumericValue::Long(0)),
            Type::Float => Some(NumericValue::Float(0.0)),
            Type::Double => Some(NumericValue::Double(0.0)),
            Type::Unsigned(t) => t.clone().to_numeric_value(),
            Type::Signed(t) => t.clone().to_numeric_value(),
            Type::Pointer(_) => Some(NumericValue::ULong(0)),
            Type::Array(_, _) => Some(NumericValue::ULong(0)), //array is just a pointer in this sense
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Char => write!(f, "char"),
            Type::Short => write!(f, "short"),
            Type::Int => write!(f, "int"),
            Type::Long => write!(f, "long"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::Unsigned(inner) => write!(f, "unsigned {}", inner),
            Type::Signed(inner) => write!(f, "signed {}", inner),
            Type::Pointer(inner) => write!(f, "{}*", inner),
            Type::Array(inner, Some(size)) => write!(f, "{}[{}]", inner, size),
            Type::Array(inner, None) => write!(f, "{}[]", inner),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub param_type: Type,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclaration {
        var_type: Type,
        name: String,
        initializer: Option<Expr>,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expr>,
        increment: Option<Box<Statement>>,
        body: Block,
    },
    While {
        condition: Option<Expr>,
        body: Block
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Block(Block),
    Expression(Expr), // Expressions are statements
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    Function(Function),
    VarDeclaration {
        var_type: Type,
        name: String,
        initializer: Option<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<TopLevel>,
}

pub struct ASTParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl ASTParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current(), Some(Token::EOF) | None)
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if let Some(token) = self.current() {
            if std::mem::discriminant(token) == std::mem::discriminant(&expected) {
                self.advance();
                Ok(())
            } else {
                Err(format!("Expected {:?}, found {:?}", expected, token))
            }
        } else {
            Err(format!("Expected {:?}, found EOF", expected))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            declarations.push(self.parse_top_level()?);
        }

        Ok(Program { declarations })
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        let var_type = self.parse_type()?;
        
        if let Some(Token::Identifier(name)) = self.current() {
            let name = name.clone();
            self.advance();

            // Check if it's a function declaration
            if matches!(self.current(), Some(Token::Delimiter(Delimiter::LeftParen))) {
                self.advance(); // consume '('
                let parameters = self.parse_parameter_list()?;
                self.expect(Token::Delimiter(Delimiter::RightParen))?;
                
                let body = self.parse_block()?;
                
                Ok(TopLevel::Function(Function {
                    return_type: var_type,
                    name,
                    parameters,
                    body,
                }))
            } else {
                // Variable declaration
                let initializer = if matches!(self.current(), Some(Token::Operator(Operator::Assign))) {
                    self.advance(); // consume '='
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                
                Ok(TopLevel::VarDeclaration {
                    var_type,
                    name,
                    initializer,
                })
            }
        } else {
            Err("Expected identifier after type".to_string())
        }
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, String> {
        let mut params = Vec::new();

        if matches!(self.current(), Some(Token::Delimiter(Delimiter::RightParen))) {
            return Ok(params);
        }

        loop {
            let param_type = self.parse_type()?;
            
            if let Some(Token::Identifier(name)) = self.current() {
                let name = name.clone();
                self.advance();
                
                params.push(Parameter { param_type, name });
                
                if matches!(self.current(), Some(Token::Operator(Operator::Comma))) {
                    self.advance();
                    continue;
                } else {
                    break;
                }
            } else {
                return Err("Expected parameter name".to_string());
            }
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let mut base_type = match self.current() {
            Some(Token::Keyword(Keyword::Void)) => {
                self.advance();
                Type::Void
            }
            Some(Token::Keyword(Keyword::Char)) => {
                self.advance();
                Type::Char
            }
            Some(Token::Keyword(Keyword::Short)) => {
                self.advance();
                Type::Short
            }
            Some(Token::Keyword(Keyword::Int)) => {
                self.advance();
                Type::Int
            }
            Some(Token::Keyword(Keyword::Long)) => {
                self.advance();
                Type::Long
            }
            Some(Token::Keyword(Keyword::Float)) => {
                self.advance();
                Type::Float
            }
            Some(Token::Keyword(Keyword::Double)) => {
                self.advance();
                Type::Double
            }
            Some(Token::Keyword(Keyword::Unsigned)) => {
                self.advance();
                let inner = self.parse_type()?;
                Type::Unsigned(Box::new(inner))
            }
            Some(Token::Keyword(Keyword::Signed)) => {
                self.advance();
                let inner = self.parse_type()?;
                Type::Signed(Box::new(inner))
            }
            _ => return Err("Expected type specifier".to_string()),
        };

        // pointers
        while matches!(self.current(), Some(Token::Operator(Operator::Multiply))) {
            self.advance();
            base_type = Type::Pointer(Box::new(base_type));
        }

        Ok(base_type)
    }

    pub fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(Token::Delimiter(Delimiter::LeftBrace))?;
        
        let mut statements = Vec::new();

        while !matches!(self.current(), Some(Token::Delimiter(Delimiter::RightBrace))) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::Delimiter(Delimiter::RightBrace))?;
        
        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current() {
            Some(Token::Delimiter(Delimiter::LeftBrace)) => {
                Ok(Statement::Block(self.parse_block()?))
            }
            Some(Token::Keyword(Keyword::If)) => {
                self.parse_if_statement()
            }
            Some(Token::Keyword(Keyword::For)) => {
                self.parse_for_statement()
            }
            Some(Token::Keyword(Keyword::While)) => {
                self.parse_while_statement()
            }
            Some(Token::Keyword(Keyword::Return)) => {
                self.advance();
                let value = if matches!(self.current(), Some(Token::Delimiter(Delimiter::Semicolon))) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                Ok(Statement::Return(value))
            }
            Some(Token::Keyword(Keyword::Break)) => {
                self.advance();
                self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                Ok(Statement::Break)
            }
            Some(Token::Keyword(Keyword::Continue)) => {
                self.advance();
                self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                Ok(Statement::Continue)
            }
            Some(Token::Keyword(_)) => {
                // Type keyword - variable declaration
                let var_type = self.parse_type()?;
                
                if let Some(Token::Identifier(name)) = self.current() {
                    let name = name.clone();
                    self.advance();

                    let initializer = if matches!(self.current(), Some(Token::Operator(Operator::Assign))) {
                        self.advance();
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };

                    self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                    
                    Ok(Statement::VarDeclaration {
                        var_type,
                        name,
                        initializer,
                    })
                } else {
                    Err("Expected identifier after type".to_string())
                }
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                match self.current() {
                    Some(Token::Operator(Operator::Assign)) => {
                        // Variable assignment
                        self.advance();
                        let value = self.parse_expression()?;
                        self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                        Ok(Statement::Assignment { name, value })
                    }
                    Some(Token::Delimiter(Delimiter::LeftParen)) => {
                        // Function call
                        self.advance();
                        let args = self.parse_argument_list()?;
                        self.expect(Token::Delimiter(Delimiter::RightParen))?;
                        self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                        Ok(Statement::FunctionCall { name, args })
                    }
                    _ => {
                        // Put the identifier back and parse as expression
                        self.pos -= 1;
                        let expr = self.parse_expression()?;
                        self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                        Ok(Statement::Expression(expr))
                    }
                }
            }
            _ => {
                // Expression statement
                let expr = self.parse_expression()?;
                self.expect(Token::Delimiter(Delimiter::Semicolon))?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect(Token::Keyword(Keyword::If))?;
        self.expect(Token::Delimiter(Delimiter::LeftParen))?;
        let condition = self.parse_expression()?;
        self.expect(Token::Delimiter(Delimiter::RightParen))?;
        
        let then_block = self.parse_block()?;
        
        let else_block = if matches!(self.current(), Some(Token::Keyword(Keyword::Else))) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.expect(Token::Keyword(Keyword::While))?;
        self.expect(Token::Delimiter(Delimiter::LeftParen))?;

        let condition = if matches!(self.current(), Some(Token::Delimiter(Delimiter::LeftParen))) {
            self.advance();
            None
        } else {
            let expr = self.parse_expression()?;
            Some(expr)
        };
        self.expect(Token::Delimiter(Delimiter::RightParen))?;
        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.expect(Token::Keyword(Keyword::For))?;
        self.expect(Token::Delimiter(Delimiter::LeftParen))?;
        
        // Init statement
        let init = if matches!(self.current(), Some(Token::Delimiter(Delimiter::Semicolon))) {
            self.advance();
            None
        } else {
            let stmt = self.parse_statement()?;
            Some(Box::new(stmt))
        };

        // Condition
        let condition = if matches!(self.current(), Some(Token::Delimiter(Delimiter::Semicolon))) {
            self.advance();
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect(Token::Delimiter(Delimiter::Semicolon))?;
            Some(expr)
        };

        // Increment
        let increment = if matches!(self.current(), Some(Token::Delimiter(Delimiter::RightParen))) {
            None
        } else {
            // Parse as assignment or expression without expecting semicolon
            let stmt = if let Some(Token::Identifier(name)) = self.current() {
                let name = name.clone();
                self.advance();
                if matches!(self.current(), Some(Token::Operator(Operator::Assign))) {
                    self.advance();
                    let value = self.parse_expression()?;
                    Statement::Assignment { name, value }
                } else {
                    // Put identifier back and parse as expression
                    self.pos -= 1;
                    let expr = self.parse_expression()?;
                    Statement::Expression(expr)
                }
            } else {
                let expr = self.parse_expression()?;
                Statement::Expression(expr)
            };
            Some(Box::new(stmt))
        };

        self.expect(Token::Delimiter(Delimiter::RightParen))?;
        let body = self.parse_block()?;

        Ok(Statement::For {
            init,
            condition,
            increment,
            body,
        })
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if matches!(self.current(), Some(Token::Delimiter(Delimiter::RightParen))) {
            return Ok(args);
        }

        loop {
            args.push(self.parse_expression()?);
            
            if matches!(self.current(), Some(Token::Operator(Operator::Comma))) {
                self.advance();
                continue;
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        // Use the existing Pratt parser for expressions
        let remaining_tokens = self.tokens[self.pos..].to_vec();
        let mut expr_parser = ExprParser::new(remaining_tokens);
        let expr = expr_parser.parse_expression(0)?;
        
        // Update our position based on how many tokens the expression parser consumed
        self.pos += expr_parser.pos;
        
        Ok(expr)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VarDeclaration { var_type, name, initializer } => {
                write!(f, "{} {}", var_type, name)?;
                if let Some(init) = initializer {
                    write!(f, " = {}", init)?;
                }
                write!(f, ";")
            }
            Statement::Assignment { name, value } => {
                write!(f, "{} = {};", name, value)
            }
            Statement::FunctionCall { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ");")
            }
            Statement::If { condition, then_block, else_block } => {
                write!(f, "if ({}) {{\n", condition)?;
                for stmt in &then_block.statements {
                    write!(f, "  {}\n", stmt)?;
                }
                write!(f, "}}")?;
                if let Some(else_blk) = else_block {
                    write!(f, " else {{\n")?;
                    for stmt in &else_blk.statements {
                        write!(f, "  {}\n", stmt)?;
                    }
                    write!(f, "}}")?;
                }
                Ok(())
            }
            Statement::For { init, condition, increment, body } => {
                write!(f, "for (")?;
                if let Some(i) = init {
                    write!(f, "{}", i)?;
                } else {
                    write!(f, ";")?;
                }
                write!(f, " ")?;
                if let Some(c) = condition {
                    write!(f, "{}", c)?;
                }
                write!(f, "; ")?;
                if let Some(inc) = increment {
                    write!(f, "{}", inc)?;
                }
                write!(f, ") {{\n")?;
                for stmt in &body.statements {
                    write!(f, "  {}\n", stmt)?;
                }
                write!(f, "}}")
            }
            Statement::While {condition, body} => {
                write!(f, "while (")?;
                if let Some(cond) = condition.clone() {
                    write!(f, "{}", cond)?;
                }
                write!(f, "){{")?;
                for stmt in &body.statements {
                    write!(f, "  {}\n", stmt)?;
                }
                write!(f, "}}")
            },
            Statement::Return(expr) => {
                write!(f, "return")?;
                if let Some(e) = expr {
                    write!(f, " {}", e)?;
                }
                write!(f, ";")
            }
            Statement::Break => write!(f, "break;"),
            Statement::Continue => write!(f, "continue;"),
            Statement::Block(block) => {
                write!(f, "{{\n")?;
                for stmt in &block.statements {
                    write!(f, "  {}\n", stmt)?;
                }
                write!(f, "}}")
            }
            Statement::Expression(expr) => write!(f, "{};", expr),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}(", self.return_type, self.name)?;
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{} {}", param.param_type, param.name)?;
        }
        write!(f, ") {{\n")?;
        for stmt in &self.body.statements {
            write!(f, "  {}\n", stmt)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::tokenize;

    #[test]
    fn test_simple_function() {
        let tokens = tokenize("int main() { return 0; }".to_string());
        let mut parser = ASTParser::new(tokens);
        let program = parser.parse_program().expect("Failed to parse");
        
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            TopLevel::Function(f) => {
                assert_eq!(f.name, "main");
                assert_eq!(f.parameters.len(), 0);
                assert_eq!(f.body.statements.len(), 1);
            }
            _ => panic!("Expected function"),
        }
    }

    #[test]
    fn test_variable_declaration() {
        let tokens = tokenize("int x = 5;".to_string());
        let mut parser = ASTParser::new(tokens);
        let program = parser.parse_program().expect("Failed to parse");
        
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            TopLevel::VarDeclaration { var_type, name, initializer } => {
                assert_eq!(name, "x");
                assert!(matches!(var_type, Type::Int));
                assert!(initializer.is_some());
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_if_statement() {
        let tokens = tokenize("int main() { if (x == 0) { return 1; } }".to_string());
        let mut parser = ASTParser::new(tokens);
        let program = parser.parse_program().expect("Failed to parse");
        
        if let TopLevel::Function(f) = &program.declarations[0] {
            assert_eq!(f.body.statements.len(), 1);
            assert!(matches!(f.body.statements[0], Statement::If { .. }));
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_for_loop() {
        let tokens = tokenize("int main() { for (int i = 0; i < 10; i = i + 1) { break; } }".to_string());
        let mut parser = ASTParser::new(tokens);
        let program = parser.parse_program().expect("Failed to parse");
        
        if let TopLevel::Function(f) = &program.declarations[0] {
            assert_eq!(f.body.statements.len(), 1);
            let init = Some(
                Box::new(Statement::VarDeclaration { 
                    var_type: Type::Int, name: String::from("i"), initializer: Some(Expr::Number(NumericValue::Long(0)))}));
            let condition = Some(
                Expr::BinaryOp { op: Token::Operator(Operator::Less), 
                    left: Box::new(Expr::Label(String::from("i"))), 
                    right: Box::new(Expr::Number(NumericValue::Long(10))) 
                }
            );
            let increment = Some(Statement::Assignment { 
                name: String::from("i"), 
                value: Expr::BinaryOp { op: Token::Operator(Operator::Assign), 
                    left: Box::new(Expr::Label(String::from("i"))), 
                    right: Box::new(Expr::Number(NumericValue::Long(1))) 
                }
            });
            let block = Block{statements: vec![Statement::Break]
            };
            assert!(matches!(f.body.statements[0].clone(), Statement::For {
                init,
                condition,
                increment,
                body
            }));
            // panic!("{:?}", f);
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_while1(){
        let tokens = tokenize("int loop_max = 100; int main() { int i = 0; while( i < loop_max ) { i += 1; } }".to_string());
        let mut parser = ASTParser::new(tokens);
        let program = parser.parse_program().expect("Failed to parse");

        let parameters = vec![];

        let body: Block = Block {
            statements: vec![
                Statement::VarDeclaration { var_type: Type::Int, name: String::from("i"), initializer: Some(Expr::Number(NumericValue::Long(0))) },
                Statement::While { condition: Some(Expr::BinaryOp { 
                    op: Token::Operator(Operator::Less), 
                    left: Box::new(Expr::Label(String::from("i"))), 
                    right: Box::new(Expr::Label(String::from("loop_max")))}), 
                    body: Block { statements: vec![Statement::Expression(Expr::BinaryOp { 
                        op: Token::Operator(Operator::PlusAssign), 
                        left: Box::new(Expr::Label(String::from("i"))), 
                        right: Box::new(Expr::Number(NumericValue::Long(1))) })] 
                    }
                }
            ]
        };

        let function = Function {return_type: Type::Int, name: String::from("main"), parameters, body};

        let program_ast: Vec<TopLevel> = vec![
            TopLevel::VarDeclaration { var_type: Type::Int, name: String::from("loop_max"), initializer: Some(Expr::Number(NumericValue::Long(100))) },
            TopLevel::Function(function)
        ];

        assert_eq!(
            program.declarations,
            program_ast
        );
    }
}
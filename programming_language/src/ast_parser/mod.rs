use crate::tokens;
use crate::tokens::*;
use crate::pratt_parser::*;

#[derive(Debug, Clone)]
pub enum AST {
    ProgramBlock {
        statements: Vec<AST>,
    },

    VariableDec {
        label: String,
        type_: tokens::Keyword,
    },

    FunctionDec {
        label: String,
        return_type: tokens::Keyword,
        params: Vec<AST>,
        statement_block: Option<Box<AST>>, //can only be a ProgramBlock
    },

    IfStatement {
        condition: Expr,
        body: Option<Box<AST>> //ProgramBlock
    },

    FunctionCall {
        label: String,
        params: Vec<Token>, //int literals or variable identifiers for now
    },

    VariableAssignment{
        name: String,
        type_: tokens::Keyword,
        value: Expr,
    },

    Return {
        expr: Option<Expr>
    },

}

use AST::*;

pub struct ASTParser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}


//won't use traits since those are slow
impl ASTParser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        let tokens = tokens.clone();
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

    fn advance_by(&mut self, amount: usize){
        if self.pos+amount < self.tokens.len(){
            self.pos += amount;
        }
    }

    fn peek_next(&mut self) -> Option<Token> {
        if self.pos + 1 < self.tokens.len() {
            return self.tokens.get(self.pos+1).cloned();
        }
        None
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }



    fn next_idx_of(&self, custom_token: Token) -> Option<usize> {
        let idx = self.pos;
        for i in idx..self.tokens.len(){
            if self.tokens[i] == custom_token{
                return Some(i)
            }
        }
        None
    }

    

    fn parse_function_dec(&mut self, type_: tokens::Keyword, label: &str) -> Result<AST, String> {
        //currently, the caller should be at the left parenthesis
        
        let mut params = Vec::new();
        let mut expect_param = false;
        //todo: collect all the function declaration parameters
        loop {
            self.advance();
            match self.current() {
                Some(Token::Keyword(type_)) => {
                    self.advance();
                    match self.current() {
                        Some(Token::Identifier(var_name)) => {
                            params.push(AST::VariableDec { label: var_name, type_: type_ });
                            expect_param = false;
                            // self.advance();
                            // match self.current() {
                            //     Some(Token::Delimiter(Delimiter::Colon)) => continue,
                            //     Some(Token::Delimiter(Delimiter::RightParen)) => break,
                            //     _ => return Err("Invalid token found after variable..".to_string())
                            // }
                        },
                        Some(Token::Delimiter(Delimiter::RightParen)) => {
                            break; //there are no parameters
                        },
                        _ => return Err("Parameter name not found after type declaration".to_string())
                    }
                }
                Some(Token::Operator(Operator::Comma)) => {
                    if expect_param {
                        return Err("multiple \",\" characters found, but no parameter declarations after them".to_string())
                    }
                    else {
                        expect_param = true
                    }
                },
                Some(Token::Delimiter(Delimiter::RightParen)) => {
                    if expect_param {
                        return Err(", found, but no parameter declaration after it".to_string())
                    }
                    break;
                }
                Some(other) => return Err(format!("Expected \")\", or a type, found {other:?}")),
                None => return Err("Function declaration missing \")\"".to_string())
            }
        }

        self.advance();
        //todo: implement forward declaration later as well (idk why I wrote this here)
        match self.current() {
            Some(Token::Delimiter(Delimiter::LeftBrace)) => (),
            other => return Err(format!("Expected \"{{\", found {other:?}"))
        }        
        //parsing the function block
        let function_statements = self.parse_statement_block()?;
        
        return Ok(AST::FunctionDec { label: label.to_string(), return_type: type_, params: params, statement_block: Some(Box::new(function_statements)) })
    }

    fn parse_variable_dec(&mut self, type_: Keyword, name: &str) -> Result<AST, String> {
        //the caller should be after the assignment operator
        let is_ptr = match self.peek_next() {
            Some(Token::Operator(Operator::Multiply)) => true,
            _ => false,
        };
        let syntax_tree;
        if is_ptr {
            panic!("pointer variables aren't implemented yet");
        }
        match type_ {
            tokens::Keyword::Int | tokens::Keyword::Float => {
                match self.current() {
                    Some(Token::Operator(Operator::Assign)) => self.advance(),
                    other => return Err(format!("Invalid token in variable declaration: {:?}", other))
                }
                
                let statement_ending = match self.next_idx_of(Token::Delimiter(Delimiter::Semicolon)) {
                    Some(n) => n,
                    None => return Err("No ; found after variable assignment".to_string())
                };
                let new_arr = Vec::from(&self.tokens[self.pos..statement_ending]);
                let mut parser = ExprParser::new(new_arr);
                let expr = parser.parse_expression(0)?;
                syntax_tree = AST::VariableAssignment { type_, name: name.to_string(), value: expr };
                self.advance_by(statement_ending-self.pos+1); //+1 for semicolon

                return Ok(syntax_tree);
            },

            _ => return Err("This isn't a valid keyword for a variable".to_string())
        }
    }

    pub fn parse_statement_block(&mut self) -> Result<AST, String> {
        //must be at {
        self.advance();
        let mut statements = Vec::new();

        loop {
            match self.current(){
                Some(Token::Keyword(Keyword::Return)) => {
                    self.advance();
                    match self.current(){
                        //return statements can only return floats and float expressions right now
                        Some(Token::FloatLiteral(n)) => {
                            let statement_ending = match self.next_idx_of(Token::Delimiter(Delimiter::Semicolon)) {
                                Some(n) => n,
                                None => return Err("No ; found after variable assignment".to_string())
                            };
                            let new_arr = Vec::from(&self.tokens[self.pos..statement_ending]);
                            let mut parser = ExprParser::new(new_arr);
                            let expr = parser.parse_expression(0)?;
                            statements.push(AST::Return { expr: Some(expr) });
                            self.advance_by(statement_ending-self.pos+1); //+1 for incoming semicolon
                        },
                        Some(Token::Delimiter(Delimiter::Semicolon)) => {
                            statements.push(AST::Return { expr: None });
                            self.advance();
                        }
                        other => return Err(format!("Invalid token found after return: {:?}", other))
                    }
                },
                Some(Token::Keyword(keyword)) => {
                    //this could be an if statement, a switch statement, etc...
                    self.advance();
                    match self.current() {
                        //for now, it can only parse variable declarations
                        Some(Token::Identifier(name)) => {
                            self.advance();
                            //cannot be a function declaration, so it's for variables
                            let parsed_var_dec = self.parse_variable_dec(keyword, &name)?;
                            statements.push(parsed_var_dec);
                        }
                        _ => return Err(format!("After keyword {:?}, a variable declaration is expected", keyword))
                    }
                }
                Some(Token::Delimiter(Delimiter::RightBrace)) => break, //when encounters end of function, break
                other => return Err(format!("Only variable assignments are supported for now. found: {:?}", other))
            }
        }


        Ok(AST::ProgramBlock { statements })
    }

    pub fn generate_ast(&mut self) -> Result<AST, String> {
        let mut full_program = Vec::new();
        loop {
            match self.current() {
                Some(Token::Keyword(type_)) => {
                    self.advance();
                    match self.current() {
                        Some(Token::Identifier(name)) => {
                            self.advance();
                            match self.current() {
                                //it's a variable assignment
                                Some(Token::Operator(Operator::Assign)) => {
                                    //parse_params could handle the variable names (if it's a multiple parameter declaration)
                                    //todo: parse the = sign, and the expression / array statement after it
                                    let parsed_ast = self.parse_variable_dec(type_, &name)?;
                                    full_program.push(parsed_ast);
                                    self.advance();
                                },
                                //it's a function (call or declaration)
                                Some(Token::Delimiter(Delimiter::LeftParen)) => {
                                    //parse_params will handle the parameters the function was called with (if it's a call)
                                    let parsed_ast = self.parse_function_dec(type_, &name)?;
                                    full_program.push(parsed_ast);
                                    self.advance();
                                }

                                _ => panic!("Unimplemented")
                            }
                        }
                        Some(any) => panic!("Unimplemented 2"),
                        None => break
                    }
                },
                Some(Token::EOF) => break,
                other => panic!("found {:?}", other)

            }
        }
        Ok(AST::ProgramBlock { statements: full_program })
    }
}
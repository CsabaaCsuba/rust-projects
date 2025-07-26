
/*
#Language Rules:

Type -> IDENTIFIER(with value)+OPEN_PAREN+CLOSE_PAREN+OPEN_CURLY_BRACES
KEYWORD (return) -> expression
expression -> int literal

 #variables:
Statement -> Type + Identifier + Assignment (= sign) + Expression + semicolon
(Expression type will equal the Type later)
Statement -> Type + Identifier + Assignment + statement
Statement -> ++ operator
Statement -> += Expr

-> It will be completely compatible with C
-> it main will map:
int main(){
    return 1;
}

-> to the linux syscalls:
global _start
_start:
    mov rax, 60
    mov rdi, 1
    syscall

*/

pub enum MyError {
    FAIL
}

mod tokens;
mod codegen;
use crate::tokens::*;
use crate::codegen::*;
mod pratt_parser;
use pratt_parser::*;


use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;

pub enum AST {
    FunctionDec {
        label: String,
        program_lines: Vec<AST>,
    },

    VariableAssignment{
        name: String,
        type_: tokens::Keyword,
        value: Expr,
    },


}

use AST::*;

pub struct ASTParser {
    pub tokens: Vec<Token>,
    pub pos: usize,

    
}

use tokens::*;

//won't use traits since those are slow
impl ASTParser {
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

    fn peek_next(&mut self) -> Option<Token> {
        if self.pos + 1 < self.tokens.len() {
            return self.tokens.get(self.pos+1).cloned();
        }
        None
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn next_semicolon_idx(&self) -> Option<usize> {
        let idx = self.pos;
        for i in idx..self.tokens.len(){
            if self.tokens[i] == Token::Delimiter(Delimiter::Semicolon){
                return Some(i)
            }
        }
        None
    }

    fn parse_function(&mut self, tokens: &Vec<Token>, type_: tokens::Keyword) -> Result<AST, String> {
        //currently, the caller should be at the left parenthesis
        todo!()
    }

    fn parse_variable(&mut self, type_: Keyword, name: &str) -> Result<AST, String> {
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
                let statement_ending = match self.next_semicolon_idx() {
                    Some(n) => n,
                    None => return Err("No ; found after variable assignment".to_string())
                };
                let new_arr = Vec::from(&self.tokens[self.pos..statement_ending]);
                let mut parser = ExprParser::new(new_arr);
                let expr = parser.parse_expression(0)?;
                syntax_tree = AST::VariableAssignment { type_, name: name.to_string(), value: expr };

                return Ok(syntax_tree);
            },

            _ => return Err("This isn't a valid keyword for a variable".to_string())
        }
    }

    fn generate_ast(&mut self) -> Result<AST, String> {
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
                                let parsed_ast = self.parse_variable(type_, &name)?;
                                return Ok(parsed_ast);
                            },
                            //it's a function (call or declaration)
                            Some(Token::Delimiter(Delimiter::LeftParen)) => {
                                //parse_params will handle the parameters the function was called with (if it's a call)
                                panic!("Function calls aren't implemented yet")
                            }

                            _ => panic!("Unimplemented")
                        }
                    }

                    _ => panic!("Unimplemented 2")
                }
            },
            _ => panic!("Unimplemented 3")

        }
    }
}


fn main() {
    let file_contents = fs::read_to_string("src/code.c").expect("src/code.c not found...");
    let tokens = tokenize(file_contents);
    print_tokens(&tokens);
    let asm_code = generate_code(tokens);
    if let Ok(asm_code) = asm_code {
        let mut file = File::create("src/code.asm").expect("failed to create file");
        file.write_all(asm_code.as_bytes()).expect("failed to write to file");

        assemble_with_nasm("src/code.asm", "src/code.o");
        link_with_ld("src/code.o", "src/code.out");
    }


}

fn assemble_with_nasm(input_path: &str, output_obj: &str) {
    let status = Command::new("nasm")
        .args(&["-f", "elf64", input_path, "-o", output_obj])
        .status()
        .expect("Failed to run nasm");

    if !status.success() {
        panic!("NASM failed on input file: {}", input_path);
    }
}

fn link_with_ld(input_obj: &str, output_exe: &str) {
    let status = Command::new("ld")
        .args(&[input_obj, "-o", output_exe])
        .status()
        .expect("Failed to run ld");

    if !status.success() {
        panic!("Linking failed on object file: {}", input_obj);
    }
}

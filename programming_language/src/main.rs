
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
        label: &'static str,
        program_lines: Vec<AST>,
    },

    VariableDec{
        type_: Keyword,
        value: Expr,
    },


}

use AST::*;

pub struct ASTParser {
    pub tokens: Vec<Token>,
    pub pos: usize,

    
}

use tokens::{Token::*, Keyword::*, Operator::*, Delimiter::*};

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

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn parse_params(tokens: &Vec<Token>) -> Result<AST, String> {

    }

    fn generate_ast(&mut self, tokens: Vec<Token>) -> Result<AST, String> {
        match self.current() {
            Some(Keyword(Void)) => {
                self.advance();
                match self.current() {
                    Some(Identifier(name)) => {
                        self.advance();
                        match self.current() {
                            //it's a variable assignment
                            Some(Equal) => {
                                //parse_params could handle the variable names (if it's a multiple parameter declaration)
                                //todo: parse the = sign, and the expression / array statement after it
                            },
                            //it's a function (call or declaration)
                            Some(LeftParen) => {
                                //parse_params will handle the parameters the function was called with (if it's a call)
                                panic!("Function calls aren't implemented yet")
                            }
                        }
                    }
                }
            },

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

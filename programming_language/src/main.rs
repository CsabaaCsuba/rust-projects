
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

use std::collections::HashMap;
use crate::tokens::*;
use crate::codegen::*;
mod pratt_parser;
use pratt_parser::*;


use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;

mod ast_parser;
use crate::ast_parser::*;
mod semantic_analyzer;


fn main() {
    let file_contents = fs::read_to_string("src/code.c").expect("src/code.c not found...");
    let tokens = tokenize(file_contents);
    print_tokens(&tokens);
    // let ast = ASTParser::new(&tokens).generate_ast().expect("Failed to parse AST");
    // println!("{:?}", &ast);
    // let asm_code = codegen2(ast).expect("Failed to generate Machine Code");
    
    // let asm_code = generate_code(tokens);
    // if let Ok(asm_code) = asm_code {
        // let mut file = File::create("src/code.asm").expect("failed to create file");
        // // file.write_all(asm_code.as_bytes()).expect("failed to write to file")
        // file.write_all(&asm_code.as_bytes()).expect("failed to write to file");

        // assemble_with_nasm("src/code.asm", "src/code.o");
        // link_with_ld("src/code.o", "src/code.out");
    // }


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

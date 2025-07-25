mod pratt_parser;
mod tokens;
use pratt_parser::*;
use tokens::*;

use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter an equation:");
        io::stdin().read_line(&mut input).expect("failed to read line");
        match tokenize(&input){
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse_expression(0){
                    Ok(expr) => {
                        if let Some(n) = expr.resolve_numeric(){
                            println!("The solution is: {n}");
                        }
                        else {
                            eprintln!("Division by 0 happened most likely...");
                        }
                    }
                    Err(error) => {
                        eprintln!("Error during parsing: {}", error);
                    }
                }
            }
            Err(error) =>{
                eprintln!("Error during tokenization: {}", error);
            }
        }
    }
}
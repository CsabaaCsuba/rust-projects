use crate::tokens::*;
use crate::MyError;

pub const ENTRY_NAME: &str = "main";

pub fn generate_code(tokens: Vec<Token>) -> Result<String, MyError>{
    let mut raw_asm = String::new();
    for mut i in 0..tokens.len(){
        match tokens[i] {
            Token::Keyword(Keyword::Int) => {
                match &tokens[i+1]{
                    Token::Identifier(func_name) => {

                        if func_name == ENTRY_NAME && 
                        tokens[i+2] == Token::Delimiter(Delimiter::LeftParen) &&
                        tokens[i+3] == Token::Keyword(Keyword::Void) &&
                        tokens[i+4] == Token::Delimiter(Delimiter::RightParen) &&
                        tokens[i+5] == Token::Delimiter(Delimiter::LeftBrace) {
                            raw_asm += "global _start\n_start:\n";
                            if tokens[i+6] == Token::Keyword(Keyword::Return) {
                                match tokens[i+7] {
                                    Token::IntLiteral(num) => {
                                        raw_asm += &format!("mov rax, 60\nmov rdi, {}\n", num);
                                        raw_asm += "syscall";
                                        i += 7;
                                        return Ok(raw_asm);
                                    }
                                    _ => panic!("int literal expected after return")
                                }
                            }
                        }
                    },

                    _ => todo!()
                }
            },
            Token::Whitespace | Token::Newline => continue,
            _ => panic!("can't select other tokens right now...")
        }
    }
    Err(MyError::FAIL)
}
mod lexer;
mod ast_v2;
use std::str::SplitWhitespace;
use crate::lexer::*;

mod tests;


fn main() {
    let src_code = "12.3 def var1 ";
    for word in src_code.split_whitespace() {
        dbg!(tokenize(word));
    }
}

mod command_parser;
#[allow(unused_imports)]
use command_parser::{tokenizer, tokens};
mod engine;

fn main() {
    command_parser::read_command_loop();
}

mod command_parser;
use command_parser::syntax_checker;
// mod engine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser(){
        
    }
}

fn main() {
    command_parser::read_command_loop();
}

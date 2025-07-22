

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token::*, lexer::{Lex}, Token, Ops::*, Ops};

    #[test]
    fn lexing_nums() {
        let input = " 2.3  4.654345   700   0.23423  ";
        let tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                Number(2.3),
                Number(4.654345),
                Number(700.0),
                Number(0.23423),
            ]
        );
    }

    #[test]
    fn lexing_identifiers() {
        let input = " var1   xyz   GLBAL   some_count ";
        let tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                Identifier(&"var1"),
                Identifier(&"xyz"),
                Identifier(&"GLBAL"),
                Identifier(&"some_count"),
            ]
        );
    }

    #[test]
    fn lexing_operators() {
        let input = " + - * / ";
        let tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                Operator(Plus),
                Operator(Minus),
                Operator(Mult),
                Operator(Div),
            ]
        );
    }
    
    #[test]
    fn lexing_calls() {
        let mut input = " func1(2, 5, 10) ";
        let mut tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                Identifier(&"func1"),
                OpenParen,
                Number(2.0),
                Comma,
                Number(5.0),
                Comma,
                Number(10.0),
                ClosedParen,
            ]
        );

        input = " func2 () ";
        tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![Identifier(&"func2"), OpenParen, ClosedParen,]
        );

        input = " func3 (x + 2) ";
        tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                Identifier(&"func3"),
                OpenParen,
                Identifier(&"x"),
                Operator(Ops::Plus),
                Number(2.0),
                ClosedParen,
            ]
        );
    }

    #[test]
    fn lexing_function_defs() {
        let mut input = " def myCalculation(arg1 arg2) ";
        let mut tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                FuncDef,
                Identifier(&"myCalculation"),
                OpenParen,
                Identifier(&"arg1"),
                Identifier(&"arg2"),
                ClosedParen,
            ]
        );

        input = " def noParamsCall ( ) ";
        tokens = input.lex();

        assert_eq!(
            tokens.collect::<Vec<Token>>(),
            vec![
                FuncDef, 
                Identifier(&"noParamsCall"), 
                OpenParen, 
                ClosedParen,
            ]
        );
    }
}
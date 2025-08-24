// use std::collections::HashMap;
// use crate::ast_parser;
// use crate::Keyword;
// use crate::Token;
// use crate::Expr;

// pub struct SymbolTable {
//     variables: HashMap<String, usize>
// }

// impl SymbolTable {
//     pub fn new() -> Self {
//         SymbolTable { variables: HashMap::new() }
//     }
// }

// pub enum AnnotatedAST {
//     ProgramBlock {
//         symbol_table: SymbolTable,
//         statements: Vec<AST>,
//     },

//     VariableDec {
//         label: String,
//         type_: Keyword,
//     },

//     FunctionDec {
//         label: String,
//         return_type: Keyword,
//         params: Vec<AST>,
//         statement_block: Option<Box<AST>>, //can only be a ProgramBlock
//     },

//     IfStatement {
//         condition: Expr,
//         body: Option<Box<AST>> //ProgramBlock
//     },

//     FunctionCall {
//         label: String,
//         params: Vec<Token>, //int literals or variable identifiers for now
//     },

//     VariableAssignment{
//         name: String,
//         type_: Keyword,
//         value: Expr,
//     },

//     Return {
//         expr: Option<Expr>
//     },
// }
use rand::Rng;
use std::{collections::HashMap, hash::Hash};

use crate::{ast_parser::{Block, Parameter, Program, Statement, TopLevel, Type}, pratt_parser::{Expr, NumericValue}};


#[derive(Debug, Clone, PartialEq)]
pub enum SymbolValue {
    None,
    //to_le_bytes().to_vec() can convert integers into a an array of bytes
    Existing(Vec<u8>)
}


#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    //usize = size of their values, SymbolValue = stores if symbol has a defined value, and the value
    scope_variables: HashMap<String, (usize, SymbolValue)>,
    children: Vec<SymbolTable>,
    scope_level: usize, //level 0 is the root
    scope_size: usize, //number of bytes stored for this scope on the stack
    scope_id: u32,
}



pub enum AnnotatedStatement {
    VarDeclaration {
        var_type: Type,
        name: String,
        initializer: Option<Expr>,
    },
    VarAssignment {
        var_type: Type,
        name: String,
        value: Expr,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expr>,
        increment: Option<Box<Statement>>,
        body: Block,
    },
    While {
        condition: Option<Expr>,
        body: Block
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Block(Block),
    Expression(Expr), // Expressions are statements
}

pub struct AnnotatedFunction {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

pub struct AnnotatedGlobalVar {
    pub _type: Type,
    pub name: String,
    pub value: Expr,
}


impl Program {
    pub fn annotated_statement(&self) -> Result<AnnotatedStatement, String> {
        let semantic_tree: AnnotatedStatement;

        let program = self.declarations.clone();
        let mut root_symbol_table = SymbolTable {
            scope_variables: HashMap::new(),
            children: Vec::new(),
            scope_level: 0,
            scope_size: 0,
            scope_id: rand::rng().random()
        };

        for item in program {
            match item {
                TopLevel::Function(f) => {

                },
                TopLevel::VarDeclaration { var_type, name, initializer } => {
                    match initializer {
                        Some(expression) => {
                            match expression.untangle_variables( 
                                root_symbol_table.scope_id,
                                root_symbol_table.scope_level,
                                 &mut root_symbol_table) {
                                Ok(mut valid_expr) => {
                                    valid_expr.coerce_to(&var_type.to_numeric_value().expect("void variables are incorrect"));
                                    
                                },
                                Err(str) => panic!("invalid global variable: {str}") 
                            }
                        },
                        None =>{root_symbol_table.scope_variables.insert(name.clone(), (var_type.size(), SymbolValue::None));}
                    }
                }
            }
        }
        
        Ok(semantic_tree)
    }
}

pub fn match_label(label: &String, own_scope_id: u32, own_scope_depth: usize, symbols: &mut SymbolTable) -> Result<Expr, String> {
    for symbol in symbols.scope_variables.keys() { //search in top level scope
        if symbol == label {
            match symbols.scope_variables.get(symbol) {
                Some((value_size, value)) => {
                    match value {
                        SymbolValue::None => return Err(format!("Variable {label} doesn't have an actual value")),
                        SymbolValue::Existing(items) => {
                            // pad with zeros up to 8 bytes
                            let mut buf = [0u8; 8];
                            let len = items.len().min(8);
                            buf[..len].copy_from_slice(&items[..len]);

                            return Ok(Expr::Number(NumericValue::ULong(
                                u64::from_ne_bytes(buf),
                            )));
                        }
                    }
                }
                None => {
                    return Err(format!("Variable {label} doesn't have a value, it cannot be used in an expression"))
                }
            }
        }
    }

    for (i, table) in symbols.children.iter().enumerate(){
        //search through all the other legit scopes

    }

    Err(format!("Couldn't find variable {label}"))

}

impl Expr {
    //returns an option since the expression might contain invalid symbols
    //also, it can only see variables in it's own scope, or scopes above that
    pub fn untangle_variables(&self, own_scope_id: u32, own_scope_depth: usize, symbols: &mut SymbolTable
    ) -> Result<Expr, String> {
        match self {
            Expr::Number(n) => Ok(self.clone()),
            Expr::Label(label) => {
                match_label(label, own_scope_id, own_scope_depth, symbols)
            },
            Expr::FunctionCall(label, parameters) => {

            },
            Expr::BinaryOp { op, left, right } => {

            }
        }
    }
}
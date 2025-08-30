use rand::Rng;
use std::{collections::HashMap, hash::Hash, thread::scope};

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

pub struct AnnotatedTopLevel {
    pub expressions: Expr,
}


impl Program {
    pub fn annotated_statement(&self) -> Result<AnnotatedTopLevel, String> {
        //just a default value, do not use this
        let mut semantic_tree: AnnotatedTopLevel = AnnotatedTopLevel { expressions: Expr::Number(NumericValue::Char(0)) };

        let program = self.declarations.clone();
        let mut root_symbol_table = SymbolTable {
            scope_variables: HashMap::new(),
            children: Vec::new(),
            scope_level: 0,
            scope_size: 0,
            scope_id: rand::rng().random()
        };

        for item in program.clone() {
            match item {
                TopLevel::Function(f) => {
                    todo!()
                },
                TopLevel::VarDeclaration { var_type, name, initializer } => {
                    match initializer {
                        Some(expression) => {
                            match expression.untangle_variables(
                                root_symbol_table.scope_id,
                                root_symbol_table.scope_level,
                                 &mut root_symbol_table,
                                &program) {
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

pub fn iter_scope(label: &String, max_depth: usize, scope_id: u32, symbol_table: &SymbolTable) -> Result<Expr, String> {
    //search for the label in the current scope
    if symbol_table.scope_level < max_depth || symbol_table.scope_id == scope_id {
        let found_label = symbol_table.scope_variables.get(label);
        match found_label.cloned() {
            Some((length, value)) => {
                match (length, value) {
                    (0, _) => return Err(format!("Parsing error at: {label} ")),
                    (_, SymbolValue::None) => return Err(format!("Symbol {label} was declared, but without a value")),
                    (_, SymbolValue::Existing(bytes)) => {
                        let mut buf = [0u8; 8];
                        let len = bytes.len().min(8);
                        buf[..len].copy_from_slice(&bytes[..len]);

                        return Ok(Expr::Number(NumericValue::ULong(
                            u64::from_ne_bytes(buf),
                        )));
                    }
                };
                
                return Ok(Expr::Label(label.clone()));
            }
            None =>(),
        }
    }
    for i in 0..symbol_table.children.len(){
        //if it isn't found yet, iterate through the current symbol table's child
        if symbol_table.children[i].children.len() != 0 &&
            (symbol_table.scope_level < max_depth || symbol_table.scope_id == scope_id){
            let e = iter_scope(label, max_depth, scope_id, &symbol_table.children[i]);
            match e {
                Ok(e) => return Ok(e),
                Err(_) => continue //do not give up, go to the next iteration
            }
        }
    }

    Err(format!("variable {label} wasn't found!"))
}

pub fn match_label(label: &String, own_scope_id: u32, own_scope_depth: usize, symbols: &mut SymbolTable) -> Result<Expr, String> {
    for symbol in symbols.scope_variables.keys() { //search in top level scope
        if symbol == label {
            match symbols.scope_variables.get(symbol) {
                Some((value_size, value)) => {
                    match value {
                        SymbolValue::None => return Err(format!("Variable {label} doesn't have an actual value")),
                        SymbolValue::Existing(bytes) => {
                            // pad with zeros up to 8 bytes
                            let mut buf = [0u8; 8];
                            let len = bytes.len().min(8);
                            buf[..len].copy_from_slice(&bytes[..len]);

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

    iter_scope(label, own_scope_depth, own_scope_id, symbols)

}

pub fn search_for_function() -> Result<Expr, String> {
    todo!()
}

impl Expr {
    //returns an option since the expression might contain invalid symbols
    //also, it can only see variables in it's own scope, or scopes above that
    pub fn untangle_variables(&self, own_scope_id: u32, own_scope_depth: usize, symbols: &mut SymbolTable,
    declarations: &Vec<TopLevel>) -> Result<Expr, String> {
        match self {
            Expr::Number(n) => Ok(self.clone()),
            //searches for valid variable declarations
            Expr::Label(label) => {
                match_label(label, own_scope_id, own_scope_depth, symbols)
            },
            Expr::FunctionCall(label, parameters) => {
                search_for_function();
                todo!()
            },
            Expr::BinaryOp { op, left, right } => {
                todo!()
            }
        }
    }
}
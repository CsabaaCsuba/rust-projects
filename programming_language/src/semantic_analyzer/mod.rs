use rand::{Rng, RngCore};
use std::{collections::HashMap, fmt::format, hash::Hash, sync::{Arc, Mutex, Weak}, thread::scope};

use crate::{ast_parser::{Block, Parameter, Program, Statement, TopLevel, Type}, pratt_parser::{Expr, NumericValue}};


#[derive(Debug, Clone)]
pub struct SymbolTable {
    //usize = size of their values, SymbolValue = stores if symbol has a defined value, and the value
    scope_variables: HashMap<String, (usize, Option<Expr>)>,
    children: Vec<SymbolTable>,
    scope_level: usize, //level 0 is the root
    scope_size: usize, //number of bytes stored for this scope on the stack
    scope_id: u32,
    parent: Option<Weak<Mutex<SymbolTable>>>,
}

impl SymbolTable {
    pub fn new(scope_level: usize, parent: Option<Arc<Mutex<SymbolTable>>>) -> Self {
        let parent = match parent {
            Some(ptr) => Some(Arc::downgrade(&ptr)),
            None => None
        };
        SymbolTable { scope_variables: HashMap::new(), children: Vec::new(), scope_level, scope_size: 0, scope_id: rand::rng().next_u32(), parent }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedBlock {
    statements: Vec<AnnotatedStatement>
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedFunction {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedGlobalVar {
    pub _type: Type,
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedTopLevel {
    pub expressions: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionRegistry {
    functions: Vec<AnnotatedFunction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedProgram {
    pub declarations: Vec<AnnotatedTopLevel>,
}

fn check_and_convert_expr(root_symbol_table: &SymbolTable, label: &String, max_depth: usize) -> Result<Expr, String> {
    if max_depth > root_symbol_table.scope_level {
        return Err(format!("Variable {} was not found in it's scope and scopes above it", label));
    }
    match root_symbol_table.scope_variables.get(label) {
        Some((size, expr)) => {

        },
        None => {

        }
    };
}

fn check_statement(stmt: Statement, declared_functions: &mut FunctionRegistry, root_symbol_table: &SymbolTable, 
    symbol_table: &mut SymbolTable) -> Result<AnnotatedStatement, String> {
    match stmt {
        Statement::VarDeclaration { var_type, name, initializer } => {
            //ensure that variable hasn't been declared yet
            let symbol_found = symbol_table.scope_variables.get(&name);
            match symbol_found {
                None => (), //good
                Some(symbol) => {
                    return Err(format!("symbol {} already found with value {:?}", name, symbol.1))
                }
            }

            //convert the expr's type
            //also ensure that the expression doesn't contain any undefined symbols
            //+ add the variable to the symbol table
            match initializer {
                Some(mut expr) => {
                    let new_expr = expr.untangle_variables(symbol_table.scope_id, symbol_table.scope_level, root_symbol_table, &declared_functions)?;
                    let numeric_type = match var_type.to_numeric_value() {
                        None => return Err(format!("No numeric value type found for {}", var_type)),
                        Some( t) => t,
                    };
                    expr.coerce_to(&numeric_type);
                    symbol_table.scope_variables.insert(name.clone(), (numeric_type.size_bytes(), Some(expr.clone())));
                    return Ok(AnnotatedStatement::VarDeclaration { var_type, name, initializer:Some(expr.clone()) });
                },
                None => {
                    symbol_table.scope_variables.insert(name.clone(), (0, None));
                    return Ok(AnnotatedStatement::VarDeclaration { var_type, name, initializer:None });
                }
            }
        },
        Statement::Assignment { name, value } => {
            //ensure that variable exists, and convert the expr's type
            //also ensure that the expression doesn't contain any undefined symbols
            
        },
        Statement::FunctionCall { name, args } => {
            //ensure that the function exists, and has the right parameters
        },
        others => todo!()
        
    }
}

impl Program {
    pub fn annotated_statement(&self) -> Result<AnnotatedProgram, String> {
        //just a default value, do not use this
        let mut semantic_tree = AnnotatedProgram { declarations: Vec::new() };

        let mut function_reg = FunctionRegistry {functions: Vec::new()};

        let program = self.declarations.clone();
        let mut root_symbol_table = Arc::new(Mutex::new(SymbolTable {
            scope_variables: HashMap::new(),
            children: Vec::new(),
            scope_level: 0,
            scope_size: 0,
            scope_id: rand::rng().random(),
            parent: None,
        }));

        for item in program.clone() {
            match item {
                TopLevel::Function(f) => {
                    //add the functions to a function registry
                    //then parse the function's block, and produce an annotatedblock
                    let annotated_function = AnnotatedFunction{
                        return_type: f.return_type,
                        name: f.name,
                        parameters: f.parameters,
                        body: Block { statements: Vec::new() }
                    };

                    function_reg.functions.push(annotated_function.clone());

                    let mut symbol_table = SymbolTable::new(1, Some(root_symbol_table));
                    let mut annotated_block = AnnotatedBlock { statements: Vec::new()};


                    for stmt in annotated_function.body.statements {
                        annotated_block.statements.push(check_statement(stmt, &mut function_reg, &root_symbol_table, &mut symbol_table)?);
                    }
                    

                },
                TopLevel::VarDeclaration { var_type, name, initializer } => {
                    match initializer {
                        Some(expression) => {
                            let root_unpacked = root_symbol_table.lock().unwrap();
                            match expression.untangle_variables(
                                &root_unpacked,
                                &root_unpacked,
                                &function_reg) {
                                Ok(mut valid_expr) => {
                                    valid_expr.coerce_to(&var_type.to_numeric_value().expect("void variables are incorrect"));
                                    
                                },
                                Err(str) => panic!("invalid global variable: {str}") 
                            }
                        },
                        None =>{
                            root_symbol_table.scope_variables.insert(name.clone(), (var_type.size(), None));
                        }
                    }
                }
            }
        }
        
        Ok(semantic_tree)
    }
}

pub fn iter_scope(label: &String, max_depth: usize, scope_id: u32, symbol_table: &SymbolTable) -> Result<Expr, String> {
    //search for the label in the current scope, and retrieve it's value
    if symbol_table.scope_level < max_depth || symbol_table.scope_id == scope_id {
        let found_label = symbol_table.scope_variables.get(label);
        match found_label.cloned() {
            Some((length, value)) => {
                match (length, value) {
                    (0, _) => return Err(format!("Parsing error at: {label} ")),
                    (_, None) => return Err(format!("Symbol {label} was declared, but without a value")),
                    (_, Some(expr)) => {
                        return Ok(expr);
                    }
                };
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

pub fn match_label(label: &String, own_scope_id: u32, own_scope_depth: usize, symbols: &SymbolTable) -> Result<Expr, String> {
    for symbol in symbols.scope_variables.keys() { //search in top level scope
        if symbol == label {
            match symbols.scope_variables.get(symbol) {
                Some((value_size, value)) => {
                    match value {
                        None => return Err(format!("Variable {label} doesn't have an actual value")),
                        Some(expr) => {
                            return Ok(expr.clone());   
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
    pub fn untangle_variables(&self, own_scope: &SymbolTable, root_symbol_table: &SymbolTable,
    function_reg: &FunctionRegistry) -> Result<Expr, String> {
        match self {
            Expr::Number(n) => Ok(self.clone()),
            //searches for valid variable declarations
            Expr::Label(label) => {
                match_label(label, own_scope.scope_id, own_scope.scope_level, root_symbol_table)
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
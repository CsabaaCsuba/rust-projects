use rand::{Rng, RngCore};
use std::{clone, collections::HashMap, fmt::format, hash::Hash, sync::{Arc, Mutex, Weak}, thread::scope};

use crate::{ast_parser::{Block, Parameter, Program, Statement, TopLevel, Type}, pratt_parser::{Expr, NumericValue},  tokens::{Token, Operator}};



#[derive(Debug, Clone)]
pub struct SymbolTable {
    //usize = size of their values, SymbolValue = stores if symbol has a defined value, and the value
    scope_variables: HashMap<String, (Type, usize, Option<Expr>)>,
    children: Vec<SymbolTable>, //this might be taken out
    scope_level: usize, //level 0 is the root
    scope_size: usize, //number of bytes stored for this scope on the stack
    scope_id: u32,
    parent: Option<Weak<Mutex<SymbolTable>>>, //the pointer to tha parent is necessary
}

impl SymbolTable {
    pub fn new(scope_level: usize, parent: Option<&Arc<Mutex<SymbolTable>>>) -> Self {
        let parent = match parent {
            Some(ptr) => Some(Arc::downgrade(ptr)),
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


fn check_statement(stmt: Statement, declared_functions: &mut FunctionRegistry, 
    symbol_table: &mut SymbolTable) -> Result<AnnotatedStatement, String> {
    match stmt {
        Statement::VarDeclaration { var_type, name, initializer } => {
            //ensure that variable hasn't been declared yet (in this scope)
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
                    let new_expr = expr.untangle_variables(symbol_table, &declared_functions)?;
                    let numeric_type = match var_type.to_numeric_value() {
                        None => return Err(format!("No numeric value type found for {}", var_type)),
                        Some( t) => t,
                    };
                    expr.coerce_to(&numeric_type);
                    symbol_table.scope_variables.insert(name.clone(), (var_type.clone(), numeric_type.size_bytes(), Some(expr.clone())));
                    return Ok(AnnotatedStatement::VarDeclaration { var_type, name, initializer:Some(expr.clone()) });
                },
                None => {
                    symbol_table.scope_variables.insert(name.clone(), (var_type.clone(), 0, None));
                    return Ok(AnnotatedStatement::VarDeclaration { var_type, name, initializer:None });
                }
            }
        },
        Statement::Assignment { name, value } => {
            //ensure that variable exists (even in upper scopes), and convert the expr's type
            //also ensure that the expression doesn't contain any undefined symbols
            let _ = match_label(&name, &symbol_table);
            

            todo!()
        },
        Statement::FunctionCall { name, args } => {
            //ensure that the function exists, and has the right parameters
            todo!()
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
        let root_symbol_table = Arc::new(Mutex::new(SymbolTable {
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

                    let mut symbol_table = SymbolTable::new(1, Some(&root_symbol_table));
                    let mut annotated_block = AnnotatedBlock { statements: Vec::new()};


                    for stmt in annotated_function.body.statements {
                        annotated_block.statements.push(check_statement(stmt, &mut function_reg, &mut symbol_table)?);
                    }
                    

                },
                TopLevel::VarDeclaration { var_type, name, initializer } => {
                    match initializer {
                        Some(expression) => {
                            let root_unpacked = root_symbol_table.lock().unwrap();
                            match expression.untangle_variables(
                                &root_unpacked,
                                &function_reg) {
                                Ok(mut valid_expr) => {
                                    valid_expr.coerce_to(&var_type.to_numeric_value().expect("void variables are incorrect"));
                                    
                                },
                                Err(str) => panic!("invalid global variable: {str}") 
                            }
                        },
                        None =>{
                            let mut table = root_symbol_table.lock().unwrap();
                            table.scope_variables.insert(name.clone(), (var_type.clone(), var_type.size(), None));
                        }
                    }
                }
            }
        }
        
        Ok(semantic_tree)
    }
}

pub fn match_label(searched_label: &String, starting_scope: &SymbolTable) -> Result<Expr, String> {
    //look for this scope first, then always go a scope upper
    for label in starting_scope.scope_variables.keys().into_iter() {
        if label == searched_label {
            let variable_value = &starting_scope.scope_variables[label];
            match &variable_value.2 {
                None => return Err(format!("Variable {label} has no assigned value")),
                Some(e) => return Ok(e.clone())
            }
        }
    }

    //if it's not found in the scope, go one scope up
    match starting_scope.parent.clone() {
        Some(parent)=> {
            if let Some(strong) = parent.upgrade() {
                if let Ok(data) = strong.lock() {
                    return match_label(searched_label, &data);
                }
            }
            panic!("Parent mutex expected to exist if the option's value is not Some");
        },
        None => return Err(format!("Symbol {searched_label} not found in any of the scopes"))
    }
    

}

pub fn search_for_function(function_name: &String, params: Vec<Expr>, function_reg: &FunctionRegistry) -> Result<Expr, String> {
    for f in function_reg.functions.clone() {
        if &f.name == function_name {
            if params.len() != f.parameters.len() {return Err(format!("the amount of parameters for the function call of {function_name} is incorrect"))}
            for i in 0..f.parameters.len() {
                match params[i] {
                    Expr::
                }
            }
        }
    }

    Err(format!("Function named {function_name} not found"))
}

impl Expr {
    //returns an option since the expression might contain invalid symbols
    //also, it can only see variables in it's own scope, or scopes above that
    pub fn untangle_variables(&self, own_scope: &SymbolTable, function_reg: &FunctionRegistry) -> Result<Expr, String> {
        match self {
            Expr::Number(n) => Ok(self.clone()),
            //searches for valid variable declarations
            Expr::Label(label) => {
                match_label(label, own_scope)
            },
            Expr::FunctionCall(label, parameters) => {
                search_for_function(label, parameters.clone(), function_reg);
                todo!()
            },
            Expr::BinaryOp { op, left, right } => {
                
                todo!()
            }
        }
    }

    //resolving functions compile time is a bit advanced, so it will be implemented later
    pub fn solve_self(&self, own_scope: &SymbolTable) -> Result<Expr, String> {
        let mut new_expr: Expr;
        match self {
            Expr::Number(n) => Ok(self.clone()),
            //searches for valid variable declarations
            Expr::Label(label) => {
                match_label(label, own_scope)
            },
            Expr::FunctionCall(label, parameters) => {
                // search_for_function(label, parameters.clone(), function_reg);
                todo!()
            },
            Expr::BinaryOp { op, left, right } => {
                match *op {
                    Token::Operator(Operator::Plus) => {

                    },
                    Token::Operator(Operator::Minus) => {

                    },
                    Token::Operator(Operator::Multiply) => {

                    },
                    Token::Operator(Operator::Divide) => {

                    }
                    _ => panic!("unimplemented binary operation")
                }

                todo!()
            }
        }
    }

}
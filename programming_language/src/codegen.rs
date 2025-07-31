
use crate::tokens::*;
use crate::*;
use crate::MyError;

const REGISTER_LIST: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];

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

pub fn expression(expr: &Expr) -> Result<String, String> {
    let mut raw_asm = String::new();


    Ok(raw_asm)
}

pub fn variable_assignment(assignment: &AST, variable_count: usize) -> Result<String, String> {
    match assignment {
        AST::VariableAssignment { name, type_, value } => {
            let type_size = match type_ {
                Keyword::Int => 4,
                Keyword::Long => 8,
                Keyword::Char => 1,
                Keyword::Float => 4,
                Keyword::Short => 2,
                other => return Err(format!("invalid type: {:?}", other))
            };
            let offset = type_size * variable_count;
            let expr_value = match value.resolve_numeric() {
                Some(n) => (n + 0.5 - ((n >= 0.5) as u8 as f64)) as i64, //fast conversion to i64
                None => return Err(format!("Cannot comprehend this numeric expression"))
            };
            return Ok(format!("mov dword ptr [rbp-{}], {}", offset, expr_value));
        },
        other => return Err(format!("invalid type: {:?}", other))
    }
}

pub fn function_declaration(function: &AST, caller_variable_count: usize) -> Result<String, String> {
    let mut raw_asm = String::new();
    let mut total_variable_space = 0;
    
    match function {
        FunctionDec { label, return_type, params, statement_block } => {
            raw_asm += &format!("{}:\n", label);
            raw_asm += "\tpush rbp\n"; //saves the current frame pointer to the stack
            raw_asm += "\tmov rbp, rsp\n"; //the new frame pointer becomes the stack pointer
            //if there are less than 7 variables, the variable values will only get placed in registers from the register list.
            //more than 7 parameters will be placed on the stack, and the function will access the previous stack frame's locations (by adding into the ptr)
            //(stack grows downwards)
            let param_count = params.len();
            let register_param_count = 6;
            for i in 0..param_count{
                if i <= register_param_count {
                    //variables can start at frame-20, because of memory alignment
                    let variable_size =match &params[i] {
                        AST::VariableDec { label, type_ } => {
                            match type_ {
                                Keyword::Int => 4,
                                Keyword::Long => 8,
                                Keyword::Char => 1,
                                Keyword::Float => 4,
                                Keyword::Short => 2,
                                other => return Err(format!("invalid type: {:?}", other))
                            }
                        },
                        _other_ast => return Err(format!("Only variable declarations can occur inside function parameters"))
                    };
                    total_variable_space += variable_size;
                    raw_asm += &format!("mov {}, dword ptr [rbp-{}]", REGISTER_LIST[i], 20+total_variable_space);
                }
                else { //when a parameter above 6 is reached
                    break; //nothing has to be done, since the variables are pushed to the stack, and can be accessed form rbp+16 (most likely)
                }
            }

            let has_return;
            //after parameters, the function's block has to be parsed
            match statement_block {
                Some(pointer) => {
                    let block = *pointer.clone();
                    let mut had_return = false;
                    match block {
                        AST::ProgramBlock { statements } => {
                            for i in 0..statements.len(){
                                match &statements[i] {
                                    AST::VariableDec { label, type_ } => {
                                        panic!("implement variable declaration and separate assignment later");
                                    },
                                    AST::VariableAssignment { name, type_, value } => {
                                        
                                    },
                                    AST::FunctionCall { label, params } => {

                                    },
                                    AST::Return { expr } => {
                                        had_return = true;
                                        match expr {
                                            Some(return_expr) => {

                                            },
                                            None => {

                                            }
                                        }
                                    },
                                    other => return Err(format!("tree {other:?} is not a statement"))
                                }
                            }
                        },
                        _else => return Err(format!("invalid program block type (it is not block)"))
                    }
                    has_return = had_return;
                }
                None => has_return = false, //not every function has to have statements. But without a return, it can only be a void function
            }


            if !has_return && *return_type != Keyword::Void {
                return Err(format!("Non-void functions must return something"));
            }


            return Ok(raw_asm);
        },
        other => return Err(format!("Input wasn't a function declaration. Instead it was: {:?}", other))
    }
}

pub fn codegen2(tree: AST) -> Result<String, String>{
    let mut raw_asm = String::new();
    let mut variable_count = 0;

    match tree {
        ProgramBlock { statements } => {
            for i in 0..statements.len(){
                match &statements[i]{
                    FunctionDec { label, return_type, params, statement_block } => {
                        let res = function_declaration(&statements[i], variable_count)?;
                        raw_asm += &res;

                        
                    },
                    VariableAssignment { name, type_, value } => {
                        variable_count += 1;
                        let res = variable_assignment(&statements[i], variable_count)?;
                        raw_asm += &res;
                    },
                    VariableDec { label, type_ } => {
                        
                    },
                    _ => return Err("Nothing else can be in the outer block of the program".to_string())
                }
            }
        },

        _ => return Err("Main program should be a program block".to_string())
    }


    Ok(raw_asm)
}
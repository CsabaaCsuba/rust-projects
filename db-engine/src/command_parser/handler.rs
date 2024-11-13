use std::thread;

use super::tokens::Token;
use super::parser::{DBCreationData, TableCreationData};

pub fn exec_op() {
    todo!()
}

pub fn infer_op(token_vector: &Vec<Token>){ //infers the operation
    let mut handles = Vec::new();

    let handle = thread::spawn(|| exec_op());
    
    handles.push(handle);

    

}

use std::collections::HashMap;

pub struct SymbolTable {
    variables: Vec<HashMap<String, usize>>
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { variables: Vec::new() }
    }
}
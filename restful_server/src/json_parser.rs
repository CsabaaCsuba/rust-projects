use std::iter::Map;

//this should work (idk how will the lifetimes be enforced tho)
#[derive(Debug, Clone)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(&'a Vec<Value<'a>>),
    Object(&'a Vec<(String, Value<'a>)>),
}

//TODO: implement

impl Value<'_> {
    fn from_str(input_str: &str) -> Self {

        Value::Null
    }
}
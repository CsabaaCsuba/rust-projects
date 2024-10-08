#[allow(non_camel_case_types)]

pub enum DataType {
    JS_NULL,
    NUMBER,
    BOOLEAN,
    STRING,
    LIST,
    JSON
}


pub struct JS_Object {
    pub object_dtype: DataType,
    
    pub values_length: u32,

    pub keys: Vec<&'static str>, //static so that strings cannot change inside the vector
    pub value_s: Vec<JS_Object>,
}

impl JS_Object {
    fn new(self){

    }
}
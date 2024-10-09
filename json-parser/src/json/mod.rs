#[allow(non_camel_case_types)]

pub enum DataType {
    JS_NULL,
    NUMBER,
    BOOLEAN,
    STRING,
    LIST,
    JSON,
}

pub union Data {
    // pub JSON_value: JS_Object,
    pub num_value: f64,
    pub boolean_value: bool,
    pub string_value: &'static str,
    // pub list_element_value: JS_Object,
}

pub struct JS_Object {
    pub object_dtype: DataType,

    pub values_length: u32,

    pub keys: Vec<&'static str>, //static so that strings cannot change inside the vector
    pub value_s: Vec<JS_Object>,
}

impl JS_Object {
    fn new_empty() -> Self {
        return Self {
            object_dtype: DataType::JS_NULL,
            values_length: 1,
            keys: vec![],
            value_s: vec![],
        };
    }
    fn new(
        obj_type: DataType,
        val_length: u32,
        keys_in: Vec<&'static str>,
        vals: Vec<JS_Object>,
    ) -> Self {
        return Self {
            object_dtype: obj_type,
            values_length: val_length,
            keys: keys_in,
            value_s: vals,
        };
    }
}

fn bruh(){
    let obj1 = JS_Object::new_empty();

}
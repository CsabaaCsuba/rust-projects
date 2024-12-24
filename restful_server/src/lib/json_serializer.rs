use crate::lib::json_parser::Value;

#[macro_export]
macro_rules! json {
    (
        $(
            $field_name:ident : $field_variable:tt
        ),+ //denoting that it can repeat multiple times: +
            //marking the need for a comma: ,
    ) => {
        let $extracted_val = json!{$field_variable};
        Value::Null
    };
    [($field_variable:tt), +] => {
        let $extracted_val = json!{$field_variable};
    };
    ($field_value:expr(,)? i64, f64) => {
        Value::Number(field_value as f64)
    };
    ($field_value:expr, bool) => {
        Value::Bool(field_value)
    };
    ($field_value:expr, String) => {
        Value::String(field_value.clone())
    };
    ()=>{ //creates a simple empty object for vectors
        Value::Object(Vec::new())
    };
}



// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn test_simple_json() {
        let variable_val = -15;

        let generated_json = json!{
            test_str: "hi",
            numbah: 3,
            val: variable_val
        };

        assert_eq!(generated_json, Value::Null);
    }

    #[test]
    fn test_empty_json() {

        let generated_json = json!{
            3
        };

        assert_eq!(generated_json, Value::Object(Vec::new()));
    }
}

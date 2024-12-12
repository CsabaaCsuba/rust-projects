#[macro_export]
macro_rules! make_json {
    // Handle empty object
    {} => {
        "{}".to_string()
    };

    // Handle single key-value pair
    { $key:tt : $value:expr } => {{
        let json_value = stringify_value($value);
        format!("{{\"{}\": {}}}", stringify!($key), json_value)
    }};

    // Handle multiple key-value pairs
    { $($key:tt : $value:expr),+ } => {{
        let mut pairs = Vec::new();
        $(
            let key = stringify!($key);
            let value = stringify_value($value);
            pairs.push(format!("\"{}\": {}", key, value));
        )+
        format!("{{ {} }}", pairs.join(", "))
    }};
}

/// Converts various types to their JSON string representations
fn stringify_value<T: std::fmt::Debug + ToString>(value: T) -> String {
    use std::any::type_name;

    match std::any::type_name::<T>() {
        // Strings should be quoted
        n if n == type_name::<&str>() || n == type_name::<String>() => {
            format!("\"{}\"", value.to_string().replace("\"", "\\\""))
        },
        // Numbers (integer types)
        n if n.contains("i") || n.contains("u") => value.to_string(),
        // Floating point
        n if n.contains("f") => value.to_string(),
        // Booleans
        n if n == type_name::<bool>() => value.to_string().to_lowercase(),
        // Fallback for other types (might need expansion)
        _ => format!("\"{}\"", value.to_string())
    }
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_macro() {
        // Test empty object
        let json_empty: String = make_json! {};
        assert_eq!(json_empty, "{}");

        // Test single integer
        let json_int: String = make_json! { number: 1 };
        assert_eq!(json_int, "{\"number\": 1}");

        // Test with variable
        let numvar = 5;
        let json_var: String = make_json! { number: numvar };
        assert_eq!(json_var, "{\"number\": 5}");

        // Test multiple fields
        let json_multi: String = make_json! { 
            number: 1, 
            name: "MyName", 
            active: true 
        };
        assert_eq!(json_multi, "{\"number\": 1, \"name\": \"MyName\", \"active\": true}");

        // Test with float
        let floatvar = 3.14;
        let json_float: String = make_json! { value: floatvar };
        assert_eq!(json_float, "{\"value\": 3.14}");
    }
}

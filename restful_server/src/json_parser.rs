#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnknownKeyword, //for not-implemented characters
    NumberFail,
    StringFail,
    ArrayFail,
    ObjectFail,
    AbruptEnd, //if the JSON-string abruptly ended
}

//this should work (idk how will the lifetimes be enforced tho)
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>), //no hashmaps are needed for average length jsons (because Rust's hashmaps start with size 0)
}

//TODO: implement

trait IsAsciiNumeric {
    fn is_ascii_numeric(&self) -> bool;
}

impl IsAsciiNumeric for char {
    fn is_ascii_numeric(&self) -> bool {
        let cloned_char = self.clone();
        cloned_char as u32 >= 48 && cloned_char as u32 <= 57 //u32 so there is no chance of ov
    }
}

impl Value {
    fn from_str(input_str: &str) -> Result<Self, ParseError> {
        let input_chars: Vec<char> = input_str.chars().collect();
        let entire_object: Result<(Value, usize), ParseError> =
            Value::parse_js_object(&input_chars, input_chars.len(), 0);

        return match entire_object {
            Ok((val, _)) => Ok(val),
            Err(err) => Err(err),
        };
    }
    fn parse_js_object(
        input_chars: &Vec<char>,
        str_len: usize,
        idx: usize,
    ) -> Result<(Value, usize), ParseError> {
        //checked here once, so no overflow can happen anyway
        if idx >= str_len {
            return Err(ParseError::AbruptEnd);
        }
        let current_char = input_chars[idx];
        if idx + 4 <= str_len && &input_chars[idx..idx + 4] == "null".chars().collect::<Vec<char>>()
        {
            println!("null value found");
            return Ok((Value::Null, idx + 4));
        } else if idx + 4 <= str_len
            && &input_chars[idx..idx + 4] == "true".chars().collect::<Vec<char>>()
        {
            println!("true boolean found");
            return Ok((Value::Bool(true), idx + 4));
        } else if idx + 5 <= str_len
            && &input_chars[idx..idx + 5] == "false".chars().collect::<Vec<char>>()
        {
            println!("false boolean found");
            return Ok((Value::Bool(false), idx + 5));
        } else if (current_char >= 48 as char && current_char <= 57 as char)
            || current_char == b'-' as char
        {
            println!("number found");
            let mut end_idx = idx + 1;
            let mut sep_found = false;
            while end_idx < str_len {
                let found_char = input_chars.get(end_idx);
                println!("current char while num parsing: {found_char:?}");
                match found_char {
                    //TODO: fix this
                    Some(c) => {
                        if c.is_ascii_numeric() {
                            //to nothing here
                        } else if *c == '.' {
                            if sep_found {
                                return Err(ParseError::NumberFail);
                            }
                            sep_found = true;
                        } else {
                            if sep_found && input_chars[end_idx - 1] == '.' {
                                return Err(ParseError::NumberFail);
                            } else {
                                println!("loop ends here.");
                                // end_idx -= 1;
                                break;
                            }
                        }
                        end_idx += 1;
                    }
                    None => {
                        if sep_found && input_chars[end_idx - 1] == '.' {
                            return Err(ParseError::NumberFail);
                        } else {
                            println!("no more characters found...");
                            break;
                        }
                    }
                }
            }
            let num_str: String = input_chars[idx..end_idx-1].iter().collect();
            let parsing_op = num_str.parse::<f64>();
            if let Ok(num) = parsing_op {
                return Ok((Value::Number(num), end_idx));
            } else {
                return Err(ParseError::NumberFail);
            }
        } else if input_chars[idx] == '"' {
            println!("found string opening");
            let mut end_idx = idx + 1;
            while end_idx < str_len {
                if input_chars[end_idx] == '"' {
                    if input_chars[end_idx - 1] != '\\' {
                        //if it isn't an escape string ender
                        let found_str = String::from_iter(&input_chars[idx + 1..end_idx]);
                        return Ok((Value::String(found_str), end_idx + 1));
                    }
                }
                end_idx += 1;
            }
            return Err(ParseError::StringFail);
        } else if input_chars[idx] == '[' {
            println!("found array opening");
            let mut end_idx = idx + 1;
            let mut arr: Vec<Value> = Vec::new();

            let mut current_val: Value;

            while end_idx < str_len {
                match input_chars.get(end_idx) {
                    Some(']') => {println!("End of array. returning...");return Ok((Value::Array(arr), end_idx))}, //for the first iteration
                    Some(_) => {
                        (current_val, end_idx) =
                            Value::parse_js_object(input_chars, str_len, end_idx)?
                    }
                    None => return Err(ParseError::AbruptEnd),
                }
                //after this either the end of array ([) or the separator (,) comes
                match input_chars.get(end_idx) {
                    Some(']') | Some(',') => {
                        arr.push(current_val);
                        end_idx += 1;
                        // println!("array ended");
                    }
                    _ => return Err(ParseError::ArrayFail),
                }
            }

            Ok((Value::Array(arr), end_idx))
        } else if input_chars[idx] == '{' {
            println!("found object opening");
            let mut end_idx = idx + 1;
            let mut arr: Vec<(String, Value)> = Vec::new();

            let mut new_row: (String, Value);

            while end_idx < str_len {
                let current_key: Value;
                let key_str: String;
                (current_key, end_idx) = match input_chars.get(end_idx) {
                    Some('}') => return Ok((Value::Object(arr), end_idx)),
                    Some(char) => Value::parse_js_object(input_chars, str_len, end_idx)?,
                    None => return Err(ParseError::AbruptEnd),
                };
                if let Value::String(string_val) = current_key {
                    key_str = string_val;
                } else {
                    return Err(ParseError::ObjectFail); //nothing else can be a key, only
                }
                match input_chars.get(end_idx) {
                    Some(':') => end_idx += 1,
                    _ => return Err(ParseError::ObjectFail), //a colon must be after a key, since that separates key-value pairs
                }
                let current_value: Value;
                (current_value, end_idx) = Value::parse_js_object(input_chars, str_len, end_idx)?;

                match input_chars.get(end_idx) {
                    Some(',') => end_idx += 1,
                    Some('}') => (),
                    Some(other_char) => {
                        eprintln!("expected character: ',' OR '}}', found character: {other_char}");
                        return Err(ParseError::ObjectFail)}, //a comma must come after a value
                    _ => {eprintln!("No character found.");return Err(ParseError::ObjectFail)}, //a comma must come after a value
                }

                new_row = (key_str, current_value);
                arr.push(new_row);
            }
            Ok((Value::Object(arr), end_idx))
        } else {
            eprintln!("Current char: {current_char:?}");
            return Err(ParseError::UnknownKeyword);
        }
    }
}

static main_str: &str = "{\
    \"name\":\"Alice\", \
    \"ageAAAAAAAAAAAAAAAAAAAAAaaa\":30, \
    \"isStudent\":false, \
    \"grades\":[85.5, 90, null, {\"semester\":\"Fall\", \"GPA\":3.8}], \
    \"address\":{ \
        \"city\":\"Wonderland\", \
        \"zipcode\":12345, \
        \"coordinates\":{ \"latitude\":51.5074, \"longitude\":-0.1278 }, \
        \"aliases\": [\"Dreamland\", \"MysticCity\", \"The Great City\"] \
    }, \
    \"Mother's name\":\"Nathalie Nathalieman\", \
    \"Father's name\":null, \
    \"siblings\":[ \
        {\"name\":\"Bob\", \"age\":25, \"isStudent\":true, \"jobs\": [{\"company\":\"TechCorp\", \"position\":\"Engineer\"}]}, \
        {\"name\":\"Clara\", \"age\":28, \"isStudent\":false, \"hobbies\":[\"painting\", \"music\"]}, \
        null \
    ], \
    \"isGraduated\":false, \
    \"courses\":{ \
        \"current\":[\"Math\", \"Physics\", \"Computer Science\"], \
        \"completed\":[ \
            {\"name\":\"History\", \"year\":2019, \"grade\":\"A-\"}, \
            {\"name\":\"Biology\", \"year\":2020, \"grade\":\"B+\", \"specialNotes\":\"Lab work was excellent\"} \
        ], \
        \"upcoming\":[\"Philosophy\", \"Astronomy\"] \
    }, \
    \"preferences\":{ \
        \"colors\":[\"blue\", \"green\", \"red\"], \
        \"food\":{ \
            \"favorite\":\"Pizza\", \
            \"disliked\":\"Broccoli\" \
        }, \
        \"contact\":{ \
            \"email\":\"alice@example.com\", \
            \"phone\":null, \
            \"notifications\":true \
        } \
    }, \
    \"account_balance\":-200.75, \
    \"lucky_numbers\":[7, 13, 22, 3.14], \
    \"complex_number\":{ \
        \"real\":1.0, \"imaginary\":-1.0 \
    }, \
    \"escaped_characters\":\"Tab:\\t, Newline:\\n, Quote:\\\"\", \
    \"nested_escapes\":\"Line Break\\r\\nAnother Line\" \
}";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_parsing() {
        assert_eq!(Ok(Value::Null), Value::from_str("null"));
    }
    #[test]
    fn test_bool_parsing() {
        assert_eq!(Ok(Value::Bool(true)), Value::from_str("true"));
        assert_eq!(Ok(Value::Bool(false)), Value::from_str("false"));
    }

    #[test]
    fn test_num_parsing() {
        assert_eq!(Ok(Value::Number(3.1)), Value::from_str("3.1"));
        assert_eq!(Ok(Value::Number(2.0)), Value::from_str("2"));
        assert_eq!(Ok(Value::Number(-11.0)), Value::from_str("-11"));
        assert_eq!(Ok(Value::Number(-11.5)), Value::from_str("-11.5"));

        assert_eq!(Err(ParseError::NumberFail), Value::from_str("-11."));
        assert_eq!(Err(ParseError::NumberFail), Value::from_str("1.."));   
    }
    #[test]
    fn test_string_parsing(){
        assert_eq!(
            Ok(Value::String(String::from("hihiha"))),
            Value::from_str("\"hihiha\"")
        );

        assert_eq!(
            Ok(Value::String(String::from("hih\\\"iha"))),
            Value::from_str("\"hih\\\"iha\"")
        );
    }
    #[test]
    fn test_array(){
        let arr = Value::Array(vec![Value::String(String::from("halohalo")), Value::String(String::from("byebye"))]);

        assert_eq!(
            Ok(arr),
            Value::from_str("[\"halohalo\",\"byebye\"]"),
        );
    }

    #[test]
    fn test_obj(){
        let example_obj = Value::Object(vec![(String::from("mynum"), Value::Number(6.0))]);
        assert_eq!(
            Ok(example_obj),
            Value::from_str("{\"mynum\":6}")
        );

        let example_obj2 = Value::Object(vec![(String::from("mynum"), Value::Number(6.0)),
        (String::from("myval"), Value::String(String::from("hi")))
        ]);
        assert_eq!(
            Ok(example_obj2),
            Value::from_str("{\"mynum\":6,\"myval\":\"hi\"}")
        );


        let example_obj3 = Value::Object(vec![(String::from("mynum"), Value::Number(6.0)),
        (String::from("myval"), Value::String(String::from("hi"))),
        (String::from("my_arr"), Value::Array(vec![Value::String(String::from("bruh")), Value::Number(4.5)])),
        ]);
        assert_eq!(
            Ok(example_obj3),
            Value::from_str("{\"mynum\":6,\"myval\":\"hi\",\"my_arr\":[\"bruh\",4.5]}")
        );
    }

}

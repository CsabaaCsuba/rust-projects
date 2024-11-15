
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnknownKeyword, //for characters like 
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
    Object(Vec<(String, Value)>),
}

//TODO: implement

trait IsAsciiNumeric {
    fn is_ascii_numeric(&self) -> bool;
}

impl IsAsciiNumeric for char {
    fn is_ascii_numeric(&self) -> bool {
        self.clone() as u32 >= 48 && self.clone() as u32 <= 57 //u32 so there is no chance of overflowing
    }   
}

impl Value {
    fn from_str(input_str: &str) -> Result<Self, ParseError> {
        let input_chars: Vec<char> = input_str.chars().collect();
        let entire_object = Value::parse_js_object(&input_chars, input_chars.len(), 0);


        entire_object
    }
    fn parse_js_object(input_chars: &Vec<char>, str_len: usize, idx: usize) -> Result<Value, ParseError> {
        let current_char = input_chars[idx];
        if idx + 4 <= str_len && &input_chars[idx..idx+4] == "null".chars().collect::<Vec<char>>(){
            return Ok(Value::Null);
        }
        else if idx + 4 <= str_len && &input_chars[idx..idx+4] == "true".chars().collect::<Vec<char>>() {
            return Ok(Value::Bool(true));
        }
        else if idx + 5 <= str_len && &input_chars[idx..idx+5] == "false".chars().collect::<Vec<char>>() {
            return Ok(Value::Bool(false));
        }
        else if idx < str_len && ((current_char >= 48 as char && current_char <= 57 as char) || current_char == b'-' as char){
            println!("number found");
            let mut end_idx = idx+1;
            let mut sep_found = false;
            while idx < str_len {
                println!("loop iterated");
                let found_char = input_chars.get(end_idx);
                match found_char {
                    Some(c) => {
                        if c.is_ascii_numeric() {

                        }
                        else if *c == '.' {
                            if sep_found {
                                return Err(ParseError::NumberFail);
                            }
                            sep_found = true;
                        }
                        else {
                            if sep_found && input_chars[end_idx-1] == '.' {
                                return Err(ParseError::NumberFail);
                            }
                            else {
                                break;
                            }
                        }
                        end_idx += 1;
                    },
                    None => {
                        if sep_found && input_chars[end_idx-1] == '.' {
                            return Err(ParseError::NumberFail);
                        }
                        else {
                            break;
                        }
                    },
                }
            }
            let num_str: String = input_chars[idx..end_idx].iter().collect();
            let parsing_op = num_str.parse::<f64>();
            if let Ok(num) = parsing_op {
                return Ok(Value::Number(num));
            }
            else {
                return Err(ParseError::NumberFail);
            }   
        }
        else if idx < str_len && input_chars[idx] == '"' {
            let mut end_idx = idx+1;
            while end_idx < str_len{
                if input_chars[end_idx] == '"' {
                    if input_chars[end_idx-1] != '\\' { //if it isn't an escape string ender
                        let found_str = String::from_iter(&input_chars[idx+1..end_idx]);
                        return Ok(Value::String(found_str));
                    }
                }
                end_idx += 1;
            }



            return Err(ParseError::StringFail);

        }


        else {
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
    fn test_null_parsing(){
        assert_eq!(Ok(Value::Null), Value::from_str("null"));
    }
    #[test]
    fn test_bool_parsing(){
        assert_eq!(Ok(Value::Bool(true)), Value::from_str("true"));
        assert_eq!(Ok(Value::Bool(false)), Value::from_str("false"));
    }

    #[test]
    fn test_num_parsing(){
        assert_eq!(Ok(Value::Number(3.1)), Value::from_str("3.1"));
        assert_eq!(Ok(Value::Number(2.0)), Value::from_str("2"));
        assert_eq!(Ok(Value::Number(-11.0)), Value::from_str("-11"));
        assert_eq!(Ok(Value::Number(-11.5)), Value::from_str("-11.5"));

        assert_eq!(Err(ParseError::NumberFail), Value::from_str("-11."));
        assert_eq!(Err(ParseError::NumberFail), Value::from_str("1.."));

        assert_eq!(Ok(Value::String(String::from("hihiha"))), Value::from_str("\"hihiha\""));

        assert_eq!(Ok(Value::String(String::from("hih\\\"iha"))), Value::from_str("\"hih\\\"iha\""));
    }
}
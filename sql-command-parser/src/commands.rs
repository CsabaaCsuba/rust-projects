use std::fmt::Error;

#[derive(Debug)]
pub enum StatementType {
    CREATE,
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
    ALTER,
    DROP,
    // GRANT,
    // REVOKE,
    TRUNCATE //to quickly remove all rows from a table??
}

pub enum SecondKeyWord {
    TABLE,
    DATABASE,

}

pub static STATEMENTS: [&'static str; 7] = ["CREATE", "READ", "INSERT", "UPDATE", "DELETE", "ALTER", "DROP"];

pub fn match_statement_type(input_str: &mut String) -> Result<StatementType, Error>{
    if let Some(first_keyword) = input_str.to_uppercase().split_whitespace().next() { //this way the first element is always returned
        match first_keyword {
            "CREATE" => return Ok(StatementType::CREATE),
            "SELECT" => return Ok(StatementType::SELECT),
            "INSERT" => return Ok(StatementType::INSERT),
            "UPDATE" => return Ok(StatementType::UPDATE),
            "DELETE" => return Ok(StatementType::DELETE),
            "ALTER" => return Ok(StatementType::ALTER),
            "DROP" => return Ok(StatementType::DROP),
            "TRUNCATE" => return Ok(StatementType::TRUNCATE),
            _ => return Err(Error)
        }

    }
    else {
        return Err(Error);
    }
}

pub fn check_create_statement_order() -> bool { //would check if the tokens (or string slices, maybe) are in the correct order


    true
}

pub fn handle_found_statement(input_str: &mut String){
    if input_str[0..6].to_uppercase() == "CREATE"{
        println!("CREATE statement found.");
    }
    else if input_str[0..6].to_uppercase() == "SELECT"{
        println!("SELECT statement found.");
    }
    else if input_str[0..6].to_uppercase() == "INSERT"{
        println!("INSERT statement found.");
    }
    else if input_str[0..6].to_uppercase() == "UPDATE"{
        println!("UPDATE statement found.");
    }
    else if input_str[0..6].to_uppercase() == "DELETE"{
        println!("DELETE statement found.");
    }
    else if input_str[0..5].to_uppercase() == "ALTER"{
        println!("ALTER statement found.");
    }
    else if input_str[0..4].to_uppercase() == "DROP"{
        println!("DROP statement found.");
    }
    else if input_str[0..8].to_uppercase() == "TRUNCATE"{
        println!("TRUNCATE statement found.");
    }
    else {
        println!("starting statement didn't match anything");
    }
}
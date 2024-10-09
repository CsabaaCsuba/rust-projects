mod common_collections;
mod enum_patterns;
mod errors;
mod loops;
mod strings_n_ownership;
mod structures;
mod types;
use std::error::Error;

//main can return a Result too, with either () or an error.
//Box<dyn Error> represents any kind of error that inherits from std::error::Error (I guess)
fn main() -> Result<(), Box<dyn Error>>{ 
    types::type_info();
    loops::use_loops();
    strings_n_ownership::borrowing_fundamentals();
    strings_n_ownership::string_types();
    strings_n_ownership::other_slices();
    structures::activate_structs();
    enum_patterns::enums_and_patterns();
    common_collections::show_collections();
    errors::error_handling();
    println!("Program finished");
    Ok(())
}

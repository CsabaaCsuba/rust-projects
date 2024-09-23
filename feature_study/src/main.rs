mod types;
mod loops;
mod strings_n_ownership;
mod structures;
mod enum_patterns;

fn main() {
    types::type_info();
    loops::use_loops();
    strings_n_ownership::borrowing_fundamentals();
    strings_n_ownership::string_types();
    strings_n_ownership::other_slices();
    structures::activate_structs();
    enum_patterns::enums_and_patterns();
    println!("Program finished");
}

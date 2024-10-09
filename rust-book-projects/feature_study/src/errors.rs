use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn error_handling() {
    // let file_opening_result = File::open("hello.txt"); //hello.txt does not exist

    // let mut file_open_success = true;

    // let greeting_file = match file_opening_result {
    //     Ok(file) => file,
    //     Err(error) => (),
    // };

    let second_greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("file cannot even be created. Error: {error:?}");
            })
        } else {
            panic!("some really weird error happened, the file exists but can't be opened.");
        }
    });

    let third_greeting_file = File::open("hello.txt").unwrap(); // not so good, crashes the program

    let fourth_greeting_file = File::open("hello.txt").expect("hello.txt cannot be found"); // better, because it gives a panic-like error message


    //if let Ok(valtozo_neve) = valamilyen result valtozo / fuggvegy meghivas {
        //dolog amit csinaljon ha az Ok-ban van egy valtozo
    //}
    if let Ok(return_string) = failing_function("username_file.txt") {
        println!("failing function was succesful");
    } else {
        println!("function failed gracefully");
    }
}

//the question mark can be used to immediately return the function if it fails
//IF the function has a result as it's return type
pub fn failing_function(file_name: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(file_name)?; //question mark would return if the function fails
    let mut username = String::new();

    username_file.read_to_string(&mut username)?; //tries reading from the file

    Ok(username) //if the function didn't return, it will, with the ok result
}

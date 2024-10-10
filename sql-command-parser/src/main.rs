use std::io;

/* 
things to implement after CREATE:
DATABASE 'database-name' CHARACTER SET "utf8" collate "utf8_hungarian_ci";
TABLE
INDEX
*/

fn main() {
    let mut input = String::new();
    println!("What is your input?");
    if let Ok(bytes_read) = io::stdin().read_line(&mut input) {
        println!("bytes read: {bytes_read}");
        println!("read input: {input}");
    }
    else {
        println!("failed to read input."); //cannot access the error message this way...
    }
}

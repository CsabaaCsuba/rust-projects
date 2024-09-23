use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let magic_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    let result = io::stdin()
        .read_line(&mut guess);

    match result {
        
    }



    println!("You guessed: {}", guess);
}

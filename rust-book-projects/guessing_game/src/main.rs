use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let magic_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("invalid input bruh");

    let mut bruh = 2;
    {
        bruh = 3;
        println!("bruh is {}", bruh);
    }
    println!("bruh is {}", bruh);

    let parsing_op = guess.parse::<i32>();
    let number = match parsing_op {
        Ok(number) => number,
        Err(_error) => 0,
    };

    if (number == magic_number){
        println!("Your guess was correct.");
    }
    else {
        println!("Your guess was incorrect. The number is {}", magic_number);
    }



    
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    Montana,
    Florida,
    Ohio,
    NewYork,
    NewJersey,
    Maine,
    RhodeIsland,
    NorthCarolina,
    SouthCarolina,
    Indiana,
    Illinois,
    Texas,
    Tenessee,
    Colorado,
    NewMexico,
    California,
    NorthDakota,
    SouthDakota,
    Virginia,
    WestVirginia,
    Hawaii,
    Wisconsin,
    Louisiana,
    Georgia,
    Missouri,
    Arkansas,
    Kentucky,
    Oregon,
    Washington,
    Utah,
    Maryland,
    Pennsylvania, // --snip--
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
#[repr(u8)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter found from {state:?}");
            25
        }

    }
}

#[allow(unused_assignments)]
pub fn enums_and_patterns() {
    let coin1 = Coin::Dime;
    println!("I have {} cents", value_in_cents(coin1.clone()));

    //Options are enums as well, containing either None, or Some() with a value. When declaring it, the Option must have a type, which will be the value's type
    let mut my_coin: Option<Coin> = None;
    my_coin = Some(Coin::Quarter(UsState::Utah));

    match my_coin {
        Some(val) => println!("Coin's value in cents: {}", value_in_cents(val)),
        None => panic!("Coin not found"),
    }

    //if let can be used to unwrap binary enums (like Options)
    let age: Option<u8> = Some(18);

    if let Some(legit_age) = age {
        println!("the age variable is working, and has the value {legit_age}");
    } else {
        println!("Age is not working");
    }

    //Coin value checking with if let
    //if let correct_type = variable {}
    if let Coin::Dime = coin1 {
        println!("found a coin which is worth 10 cents, so it is a {coin1:?}");
    } else if let Coin::Quarter(state) = coin1 {
        println!("else if works. Also, the coin is a quarter of a dollar, and is from the state {state:?}");
    } else {
        println!("found coin was not a dime, neither a quarter");
    }
}

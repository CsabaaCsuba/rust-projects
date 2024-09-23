#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if (self.width > other.width && self.height > other.height)
            || (self.height > other.width && self.width > other.height)
        {
            return true;
        }
        false
    }
}

pub fn activate_structs() {
    let user1 = User {
        active: false,
        username: String::from("jóska"),
        email: "joska@gmail.com".to_string(),
        sign_in_count: 3,
    };

    let user2 = User {
        email: String::from("joska2@freemail.hu"),
        ..user1 //javascript-like value "inheritance" is valid
    };
    //dbg! macro can be used to both print and return a value

    println!("User2's properties: {user2:?}");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 25,
    };

    if rect1.can_hold(&rect2) {
        println!("rectangle 1 can hold rectangle 2");
    }
}

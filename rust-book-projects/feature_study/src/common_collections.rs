pub fn show_collections() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &mut v[0];

    //POINTERS IN RUST OMG
    *first = 5;

    println!("The first element is: {first}");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //increment the values based on a pointer
    }

    println!("V: {v:?}");

    let data = "initial content";

    let s = data.to_string();

    let s = "initial_content".to_string();

    let mut s = String::from("halo");

    let slice = " szia";

    s.push_str(slice);

    println!("string is {s}");

    let s1 = String::from("Hello ");
    let s2 = String::from("World");

    let s3 = s1.clone() + &s2; //clone is needed, else s3 would consume s1

    println!("string 3: {s3}, 1: {s1}");

    for c in "őű".chars() {
        println!("{c}");
    }

    for b in "őű".bytes() {
        println!("{b}");
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("key: {key}, value: {value}");
    }

    scores.insert("red".to_string(), 100);

    scores.insert("Blue".to_string(), 500);

    for (key, value) in &scores {
        println!("key now: {key}, value: {value}");
    }

    scores.entry(String::from("Black")).or_insert(1000);

    //A way to update hashmap's value based on an old value (which might not exist)
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let mut map = std::collections::HashMap::new();

    map.insert(8, "asd".to_string());

    let myval_res = map.get(&8);

    let myval = match myval_res {
        Some(val) => val,
        _ => &"".to_string(),
    };

    println!("myval: {myval:?}");

    let anotherval = map.entry(11).or_insert("bruh".to_string());

    println!("an other value: {anotherval:?}");
}

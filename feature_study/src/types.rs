use core::str;
use std::usize::MAX;

#[allow(unused_variables)]
pub fn type_info() {
    //primitive_types
    let x: u64 = 59;
    let u: i32 = 660;
    let y: f64 = 0.66;

    let z: usize = MAX;

    let largest_num: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_727;

    println!("largest num: {}", largest_num);

    let pair = ('a', 17);
    println!("tuple contents: {}, {}", pair.0, pair.1);

    let basic_str = "halohalo";
    println!("length: {}", basic_str.len());

    let actual_str = String::from("acute stomach pain");

    let rusts_list = Vec::from([1, 2, 3]);
    println!("Vector values: {:?}", rusts_list);

    //really weird unpacking operation

    if let Ok(s) = str::from_utf8(&[240, 159, 141, 137]) {
        println!("valid utf8 string: {}", s);
    }
}

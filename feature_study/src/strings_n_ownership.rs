#[allow(non_snake_case)]
pub fn borrowing_fundamentals(){
    let mut S1 = String::from("halohalo");

    let S2 = &S1; //S2 has the same underlying string pointer as S1

    println!("S2 and S1 are completely valid: {S2} {S1}");

    //S2 is used once, and will die because there is a mutable reference declaration
    let S2 = &mut S1;
    S2.push('!');
    println!("S1 after modifying the mutable reference: {S1}");

    let S2 = S1.clone(); //S1's character array pointer is duplicated and S2 also has the value

    println!("S2 and S1 are still completely valid: {S2} {S1}");


    let S2 = S1; //the values of S1 moved to S2, the original S1 is no longer valid
    println!("S1 has moved, so the only valid variable is S2: {S2}");

}

pub fn string_types(){
    let MyString = String::from("Yeah this is a string");

    let slice = &MyString[15..]; //[num1..num2] is the base syntax, but the numbers can be omitted 

    println!("string slice: {slice}");

    let slice2 = "this is a string slice";

    println!("An other way to declare a string slice: {slice2}");
}

pub fn other_slices(){
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let vec_slice = &num_vec[2..5];

    assert_eq!(vec_slice, [3, 4, 5]);

    println!("Vector slice: {vec_slice:?}");

    let basic_arr = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let array_slice = &basic_arr[3..9];

    

    println!("Array slice: {array_slice:?}");

}
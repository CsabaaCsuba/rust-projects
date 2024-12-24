pub fn test_challenge(){
    // let case1 = String::from("asaadc");
    // println!("Longest substring from {} is {}", case1.clone(), length_of_longest_substring(case1));

    // let case2 = String::from("pwwkew");
    // println!("Longest substring from {} is {}", case2.clone(), length_of_longest_substring(case2));

    let case3 = String::from("abcabcbb");
    println!("Longest substring from {} is {}", case3.clone(), length_of_longest_substring(case3));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest_substring_len: i32 = 0;
    let mut current_substring = String::new();
    let mut found = false;
    for char in s.chars() {
        for appended_char in current_substring.chars(){
            if char == appended_char {
                println!("string already contains character {}", appended_char);
                found = true;
                let current_len = current_substring.len() as i32;
                println!("current length: {}, current longest length: {}", current_len, longest_substring_len);
                if current_len > longest_substring_len {
                    longest_substring_len = current_len;
                }
                current_substring.remove(0);

                break;
            }
        }
       
            current_substring.push(char);
        

        println!("current substring length: {}", current_substring.len());
        found = false;
    }

    let current_len = current_substring.len() as i32;
    println!("current length: {}, current longest length: {}", current_len, longest_substring_len);
    if current_len > longest_substring_len {
        longest_substring_len = current_len;
        
    }

    return longest_substring_len;
}
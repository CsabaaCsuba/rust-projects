
pub fn test_challenge(){
    println!("returned number: {}\n", my_atoi(String::from("555")));
    println!("returned number: {}\n", my_atoi(String::from("-312")));
    println!("returned number: {}\n", my_atoi(String::from("1337c0d3")));
    println!("returned number: {}\n", my_atoi(String::from("-91283472332"))); //much smaller than min
    println!("returned number: {}\n", my_atoi(String::from("2147483648"))); //larger than max by 1
    println!("returned number: {}\n", my_atoi(String::from("2147483646"))); //smaller than max by 1
    println!("returned number: {}\n", my_atoi(String::from("21474836460")));
    println!("returned number: {}\n", my_atoi(String::from("-2147483647"))); //one larger than the limit
}

fn char_to_num(character: u8) -> i32 {
    return (character as i32)-48;
}

fn my_atoi(s: String) -> i32 {
    let mut num: i32 = 0;
    let mut negative = false;
    let mut whitespace_iteration = true;

    let mut i = 0;
    for char in s.clone().into_bytes(){
        if char >= b'0' && char <= b'9' {
            if whitespace_iteration {
                whitespace_iteration = false;
            }
            let current_num = char_to_num(char);
            
            if num > (i32::MAX / 10)-9{ //if number is greater than one tenth of the remaining number space
                println!("current number near overflow: {} entire string: {}", num, s.clone());
                if negative{
                    if let Some(char) = s.clone().bytes().nth(i+1){ //if the number is long -> there is an other char
                        let next_char = char_to_num(char);
                        if next_char >= 0 && next_char <= 9 { //IF there is a next char AND it's a number, return MAX
                            println!("in the long number case");
                            return i32::MIN;
                        }
                    }

                    if num > (i32::MAX / 10) { //if number is much larger than the limit
                        return i32::MIN;
                    }

                    if current_num <= 8 { //if number is close to the limit, but less than that
                        num = num * 10;
                        println!("current num: {}", current_num);
                        return (num*(-1)) - current_num;
                    }
                    
                    return i32::MIN;
                }
                else {
                    if let Some(char) = s.clone().bytes().nth(i+1){ //if the number is long -> there is an other char
                        let next_char = char_to_num(char);
                        if next_char >= 0 && next_char <= 9 { //IF there is a next char AND it's a number, return MAX
                            return i32::MAX;
                        }
                    }

                    if num > (i32::MAX / 10) { //if number is much larger than the limit
                        return i32::MAX;
                    }
                    if current_num <= 7 { //if number is close to the limit, but less than that
                        num = num * 10;
                        return num + current_num;
                    }

                    return i32::MAX;
                }
            }
            num = num * 10;
            num += current_num;
            i += 1;
        }
        else if whitespace_iteration{
            if char == b'-'{
                negative = true;
                println!("negative number detected.");
                whitespace_iteration = false;
                i +=1;
            }
            else if char == b'+' {
                negative = false;
                whitespace_iteration = false;
                i += 1;
            }
            else if char == b' ' || char == b'\t' || char == b'\r' || char == b'\n' {
                //if it's an early, whitespace iteration, nothing has to be done
            }
            else {
                break;
            }   
        }
        else {
            break;
        }
    }

    if negative {
        return num * (-1);
    }

    return num;
}
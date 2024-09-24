pub fn use_loops() {
    let mut i = 1;
    while i < 10 {
        println!("looping... i value: {}", i);
        i += 1;
    }

    for j in 12..=20 {
        println!("looping using for loop. j value: {}", j);
    }

    let mut n = 0;
    loop {
        println!("n value in loop: {}", n);
        n += 1;

        if n == 5 {
            break;
        }
    }

    //breaking a loop with a value
    let mut k = 1;
    let number = loop {
        k += 3;

        if k > 100 {
            break k * 2;
        }
    };

    println!("the number in the end: {}", number);

    //using loop label
    let mut count = 0;
    'counting_up: loop {
        println!("current count: {}", count);
        count += 2;

        if count > 30 {
            break 'counting_up;
        }
        'counting_down: loop {
            println!("Counting down. current count: {}", count);
            i -= 1;

            if count > 30 {
                break 'counting_down;
            } else {
                continue 'counting_up;
            }
        }
    }
    println!("count in the end: {}", count);

    //looping through a collection
    let a = [10, 20, 30, 40, 50];
    let length = a.len();

    for l in 0..=length - 1 {
        println!("collection element: at index {}: {}", l, a[l]);
    }
}

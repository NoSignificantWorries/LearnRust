fn main() {
    // Conditions
    let n = 10;

    if n % 2 == 0 {
        println!("Number is even");
    } else {
        println!("Number is odd");
    }

    if n <= 9 {
        println!("It's a digit");
    } else if n == 10 {
        println!("It's dec");
    } else {
        println!("It's numeric");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Loops
    // labels
    'l1: for num in 0..=5 {
        for num_2 in 3..=6 {
            if num == num_2 {
                println!("Equal number is: {}", num);
                break 'l1;
            }
        }
    }

    // loop
    let mut i = 0;
    loop {
        i += 1;
        if i > 123 && i % 123 == 121 {
            println!("loop ended with: {}", i);
            break;
        }
    }

    let loop_results = loop {
        i += 1;
        if i > 1000 {
            break i;
        }
    };
    println!("loop ended and returned: {}", loop_results);

    let loop_results = 'l1: loop {
        i += 1;
        loop {
            i /= 2;
            if i % 3 == 2 {
                break 'l1 i;
            }
        }
    };
    println!("upper loop ended and returned: {}", loop_results);

    // while
    let mut j = 0;
    while j <= 5 {
        println!("j: {}", j);
        j += 1;
    }

    // for
    let arr = [13, 22, 30, 40];
    for element in arr {
        println!("arr element: {}", element);
    }

    for k in (1..=4).rev() {
        println!("k: {}", k);
    }
}

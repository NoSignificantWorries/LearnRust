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

    'l1: for num in 0..=5 {
        for num_2 in 3..=6 {
            if num == num_2 {
                println!("Equal number is: {}", num);
                break 'l1;
            }
        }
    }
}

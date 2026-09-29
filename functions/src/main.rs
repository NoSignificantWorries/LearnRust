fn hello() {
    println!("Hello!");
}

fn sum(a: f64, b: f64) -> f64 {
    a + b
}

fn main() {
    hello();

    let r = sum(8.0, 17.3);

    println!("{}", r);

    let y = {
        let mut x = 3;
        x += 1;
        x
    };

    println!("{}", y);
}

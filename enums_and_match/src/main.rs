#[derive(Debug)]
enum IpAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

fn main() {
    let addr = IpAddr::IPv6(String::from("::1"));

    println!("{:?}", addr);

    let num1: Option<i32> = Option::Some(60);
    let num2 = 34;

    let sum = match num1 {
        None => None,
        Some(n) => Some(n + num2),
    };

    println!("{:?}", sum);

    if let Some(number) = num1 {
        println!("This number is {}", number);
    }
}

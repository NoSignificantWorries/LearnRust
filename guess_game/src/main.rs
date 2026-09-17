use rand::Rng;
use std::io;

fn main() {
    println!("Hello in the guess game!");
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100); // range - отдельный тип в rust

    println!("The secret number is {secret}");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read string!"); // Обработка ошибки в rust

    println!("You guessed '{guess}'.");
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello in the guess game!");
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100); // range - отдельный тип в rust

    // println!("The secret number is {secret}");

    // бесконечный цикл
    loop {
        let mut guess = String::new();
        println!("Please enter your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read string!"); // Обработка ошибки в rust

        let guess: u32 = match guess.trim().parse() {
            // в качестве результата функции в основном возвращат энам Result
            // этот энам принимает значения Ok и Err
            Ok(num) => num,
            Err(_) => continue,
        };
        //также тут использовалось затенение, мы можем перезаписать переменную с другим типом
        // trim - убирает лишние пробелы по краям
        // parse - преобразует тип, использует тип переменной

        println!("You guessed '{guess}'.");

        // match позволяет обрабатывать энамы
        match guess.cmp(&secret) {
            // варианты энама обрабатываются в ветвях, каждая ветвь содержит паттерн и код, который надо выполнить
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
                // прерываем цикл если выиграли
            }
        }
    }
}

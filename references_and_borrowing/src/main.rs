fn find_subword(text: &str) -> &str {
    let mut start: usize = 0;
    let mut in_word = false;
    for (i, &symbol) in text.as_bytes().iter().enumerate() {
        if symbol == b' ' && in_word {
            return &text[start..i];
        } else if symbol != b' ' && !in_word {
            in_word = true;
            start = i;
        }
    }
    text
}

fn main() {
    let s = String::from("  Something wrong!");

    let first_word = find_subword(&s);

    println!("First word is: {}", first_word);

    let arr = [1, 2, 3, 4, 5];

    let arr_slice = &arr[1..=3];

    println!("\nElements:");
    for elem in arr_slice {
        println!("{}", elem);
    }
}

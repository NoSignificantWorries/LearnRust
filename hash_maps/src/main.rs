use std::collections::HashMap;

fn get_words(text: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace().into_iter() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    map
}

fn main() {
    let mut dict = HashMap::new();

    dict.insert(String::from("First"), 10);
    dict.insert(String::from("Second"), 20);

    println!("{:?}", dict);

    let key = String::from("First");
    let value = dict.get(&key).copied().unwrap_or(0);

    println!("{}", value);

    for (key, value) in &dict {
        println!("{}: {}", key, value);
    }

    let key = String::from("Third");
    dict.entry(key).or_insert(30);

    println!("{:?}", dict);

    let text = "hello world wonderful world";
    let words = get_words(text);
    println!("{:?}", words);
}

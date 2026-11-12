fn find_largest<T>(list: &[T]) -> &T {
    let mut grt_pointer = &list[0];
    for elem in list {
        if elem > grt_pointer {
            grt_pointer = elem;
        }
    }
    grt_pointer
}

fn main() {
    let arr = vec![12, 32, 5, 16, 100, 43, 17];

    let largest = find_largest(&arr);

    println!("{}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
}

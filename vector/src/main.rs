fn main() {
    let vec1: Vec<i32> = Vec::new();
    println!("v1: {:?}", vec1);

    let vec2 = vec![1, 2, 3];
    println!("v2: {:?}", vec2);

    let mut vec3: Vec<i32> = Vec::new();

    for i in 0..10 {
        vec3.push(i);
    }
    println!("v3: {:?}", vec3);

    let v4 = vec![1, 2, 3, 4, 5];

    let second: &i32 = &v4[1];
    println!("Second element: {}", second);

    let third: Option<&i32> = v4.get(2);

    match third {
        Some(value) => println!("Third value is {}", value),
        None => println!("No third element in the vector!"),
    }

    let mut v5 = vec![1, 2, 3, 4, 5, 6];

    for v in &v5 {
        println!("Element: {}", v);
    }

    for v in &mut v5 {
        *v += 5;
    }
    println!("v5: {:?}", v5);
}

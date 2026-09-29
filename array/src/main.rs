fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", arr[0]);

    let arr = [3; 5];

    println!("{}", arr[3]);
    // println!("{}", arr[10]); // not compiling
}

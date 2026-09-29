fn main() {
    let mut point = (23, 56, -89);

    point.0 -= 8;

    let (mut x, y, z) = point;

    println!("Point (x={}, y={}, z={})", x, y, z);

    x += 2;

    println!("Point (x={}, y={}, z={})", x, y, z);
}

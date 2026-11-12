use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {
        variety: String::from("something"),
    };
    println!("Vegetable: {:?}", plant);
}

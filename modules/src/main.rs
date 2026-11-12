mod test {
    // pub делает функцию публичной
    pub fn testing() {
        println!("This is test!");
        inner_test(); // можем  использовать приватные функции внутри поля
    }

    fn inner_test() {
        println!("This is inner test!");
    }
}

fn main() {
    test::testing();
    // не можем обратиться к приватным полям
    // test::inner_test();
}

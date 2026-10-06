#[derive(Debug)]
struct User {
    active: bool,
    login: String,
    email: String,
}

fn make_user(login: String, email: String) -> User {
    User {
        active: true,
        login,
        email,
    }
}

fn main() {
    let user1 = User {
        active: true,
        login: String::from("Null"),
        email: String::from("null.unknown@none.com"),
    };

    println!("{:?}", user1);

    let user2 = make_user(
        String::from("Second"),
        String::from("second.double@gmail.com"),
    );

    println!("{:?}", user2);

    let user3 = User {
        email: String::from("new.new@nmail.com"),
        ..user1
    };

    println!("{:?}", user3);
}

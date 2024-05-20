struct User {
    active: bool,
    name: String,
}

pub fn execute_stuff() {
    let user1 = User {
        active: false,
        name: String::from("user1"),
    };

    println!("{}, {}", user1.active, user1.name);
}

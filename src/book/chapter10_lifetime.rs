pub fn execute_lifetime() {
    let a = String::from("hello");
    let b = String::from("way looooooooooonger");
    let long = longest(&a, &b);
    println!("{}", long);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


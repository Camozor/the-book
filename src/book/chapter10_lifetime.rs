use std::fmt::Display;

pub fn execute_chapter10_lifetime() {
    execute_lifetime();
    execute_struct_lifetime();
    execute_announce_and_longest();
}

pub fn execute_lifetime() {
    let a = String::from("hello");
    let b = String::from("way looooooooooonger");
    let long = longest(&a, &b);
    println!("{}", long);
}

// Does not compile, missing lifetime
// fn longest_fail(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportExcerpt<'a> {
    #[allow(dead_code)]
    fn level(&self) -> i32 {
        3
    }

    #[allow(dead_code)]
    fn announce_and_return_part(&self, announcment: &str) -> &str {
        println!("{announcment}");
        self.part
    }
}

fn execute_struct_lifetime() {
    let reference;
    let s = String::from("Hello world. How are you?");
    reference = s.split('.').next().expect("Could not split with char '.'");

    let excerpt = ImportExcerpt { part: reference };
    print!("{}", excerpt.part);
}

fn execute_announce_and_longest() {
    let a = String::from("hello");
    let b = String::from("world");
    announce_and_longest(&a, &b, String::from("display"));
}

fn announce_and_longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::thread;

pub fn execute_chapter13() {
    // execute_inventory();
    // execute_closure_ownership();
    execute_closure_thread();
}

#[allow(dead_code)]
fn execute_inventory() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };
    println!("{:?}", store.most_stocked());
    println!(
        "Give away for blue preference {:?}",
        store.giveaway(Some(ShirtColor::Blue))
    );
    println!("Give away for no preference {:?}", store.giveaway(None));
}

#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn most_stocked(&self) -> ShirtColor {
        let mut red_number = 0;
        let mut blue_number = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_number += 1,
                ShirtColor::Blue => blue_number += 1,
            }
        }

        if red_number > blue_number {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

#[allow(dead_code)]
fn fails() {
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    // let _n = example_closure(5); // doesn't compile 5 is not a string
}

#[allow(dead_code)]
fn execute_closure_ownership() {
    // Only borrowing
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Borrowing with mut
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

#[allow(dead_code)]
fn execute_closure_thread() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

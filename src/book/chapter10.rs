use std::fmt::{Debug, Display};

#[allow(dead_code)]
pub fn execute_chapter10() {
    // execute_largest();
    // execute_point();
    // execute_trait();
    execute_pair_trait();
}

#[allow(dead_code)]
fn execute_largest() {
    let l = vec![1, 3, 7, 5, 3];
    println!("Largest={}", largest(&l));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for element in list {
        if element > largest {
            largest = element;
        }
    }

    largest
}

#[allow(dead_code)]
fn execute_point() {
    let point = Point { x: 4, y: 7 };

    println!("Point: {} {}", point.x, point.y);

    // let a = Point { x: 4, y: 5.2 }; // Illegal
}

#[allow(dead_code)]
fn execute_trait() {
    let article = Article {
        headline: "The headline".to_string(),
        author: "Titi".to_string(),
    };
    println!("{}", article.summarize());

    article.other_stuff();

    let tweet = Tweet {
        author: "Titi".to_string(),
    };
    tweet.other_stuff();

    notify(&article);
    notify(&tweet);
}

struct Point<T> {
    x: T,
    y: T,
}

trait Summary {
    fn summarize(&self) -> String;

    fn other_stuff(&self) {
        println!("Other stuff");
    }
}

struct Article {
    headline: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} {}", self.author, self.headline)
    }
}

struct Tweet {
    author: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.author)
    }

    fn other_stuff(&self) {
        println!("Overriden other stuff");
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Syntactic sugar for this
#[allow(dead_code)]
fn alternative_notify<T: Summary>(item: &T) {
    println!("Alternative breaking news! {}", item.summarize());
}

// Clean syntax
#[allow(dead_code)]
fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

#[allow(dead_code)]
fn execute_return_trait() {
    let _val = returns_summarizable(); // _val is typed "impl Summary"
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        author: "Titi".to_string(),
    }
}

#[allow(dead_code)]
fn execute_pair_trait() {
    let pair = Pair::new(17.2, 5.6);
    pair.cmp_display();

    let pair2 = Pair::new("hello", "world");
    pair2.cmp_display();

    let pair3 = Pair::new(true, false);
    pair3.cmp_display();

    let _pair4 = Pair::new(Bidule {}, Bidule {});
    // _pair4.cmp_display(); // Illegal trait bound not satisfied for Bidule
}

struct Pair<T> {
    x: T,
    y: T,
}

struct Bidule {}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

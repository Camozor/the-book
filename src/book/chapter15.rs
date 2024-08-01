use std::{ops::Deref, usize};

#[allow(dead_code)]
pub fn execute_chapter15() {
    execute_my_box();
}

// A kind of smart pointer
#[allow(dead_code)]
fn execute_box() {
    let b = Box::new(1);
    println!("{}", b);
}

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    fn size(&self) -> usize {
        match self {
            Self::Nil => 0,
            Self::Cons(_, l) => 1 + l.size(),
        }
    }
}

#[allow(dead_code)]
fn execute_cons_list() {
    let l2 = List::Cons(5, Box::new(List::Nil));
    let l3 = List::Cons("a", Box::new(List::Cons("b", Box::new(List::Nil))));

    println!("{:?}", l2);
    println!("{:?}", l3);

    println!("{}", l3.size());
}

#[allow(dead_code)]
fn execute_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
fn execute_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
}

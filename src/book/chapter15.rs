use std::{
    ops::{Deref, DerefMut},
    usize,
};

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
        println!("deref called");
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop the MIC");
    }
}

#[allow(dead_code)]
fn execute_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    println!("assert_eq 1");
    assert_eq!(5, *y);
    println!("assert_eq 2");
    assert_eq!(5, *y);

    println!("Dropping y");
    drop(y);
    println!("Dropped y");

    // println!("{}", *y); // Borrow of moved value
}

struct MyMutBox(i32);
impl MyMutBox {
    fn new(x: i32) -> Self {
        Self(x)
    }

    fn mutate(&mut self, x: i32) {
        self.0 = x;
    }
}

impl DerefMut for MyMutBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for MyMutBox {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
fn execute_my_mut_box() {
    let mut mut_box = MyMutBox::new(7);
    println!("{}", *mut_box);
    mut_box.mutate(8);
    assert_eq!(8, *mut_box);
    println!("{}", *mut_box);

    *mut_box = 9;
    println!("{}", *mut_box);
}

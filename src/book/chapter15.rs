use std::usize;

#[allow(dead_code)]
pub fn execute_chapter15() {
    create_cons_list();
}

// A kind of smart pointer
#[allow(dead_code)]
fn create_box() {
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

fn create_cons_list() {
    let l2 = List::Cons(5, Box::new(List::Nil));
    let l3 = List::Cons("a", Box::new(List::Cons("b", Box::new(List::Nil))));

    println!("{:?}", l2);
    println!("{:?}", l3);

    println!("{}", l3.size());
}

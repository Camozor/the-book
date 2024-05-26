struct User {
    active: bool,
    name: String,
}

#[allow(dead_code)]
pub fn execute_chapter5() {
    // execute_struct();
    // execute_tuple_struct();
    // execute_area();
    // execute_can_hold();
    // execute_square();
}

#[allow(dead_code)]
fn execute_struct() {
    let user1 = User {
        active: false,
        name: String::from("user1"),
    };
    println!("{}, {}", user1.active, user1.name);

    let user2 = build_user(String::from("user2"));
    println!("{}, {}", user2.active, user2.name);

    let user3 = User {
        active: false,
        name: user2.name,
    };
    let _user4 = User {
        active: true,
        ..user3
    };
    // println!("{}", user3.name); // Moved !!
}

fn build_user(name: String) -> User {
    User {
        active: false,
        name,
    }
}

struct Color(u8, u8, u8);

#[allow(dead_code)]
fn execute_tuple_struct() {
    let white = Color(255, 255, 255);
    println!("{}, {}, {}", white.0, white.1, white.2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
fn execute_area() {
    let r = Rectangle {
        width: 3,
        height: 5,
    };

    println!("r is {:#?}", r);
    // dbg!(&r); // Debug macro

    println!("{}", compute_area(&r));
    println!("{}", r.area());
}

#[allow(dead_code)]
fn execute_can_hold() {
    let r1 = Rectangle {
        width: 3,
        height: 5,
    };
    let r2 = Rectangle {
        width: 2,
        height: 5,
    };
    let r3 = Rectangle {
        width: 4,
        height: 2,
    };

    println!("r1 can hold r2 {}", r1.can_hold(&r2));
    println!("r1 can hold r3 {}", r1.can_hold(&r3));
}

#[allow(dead_code)]
fn execute_square() {
   let r = Rectangle::square(45);
    println!("r = {:?}", r);
}

#[allow(dead_code)]
fn compute_area(r: &Rectangle) -> u32 {
    r.width * r.height
}

#[allow(dead_code)]
impl Rectangle {
    // "&self" alias for "self: &Self"
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Self) -> bool {
        self.width >= rect2.width && self.height >= rect2.height
    }

    fn square(length: u32) -> Self {
        Self { width: length, height: length }
    }
}

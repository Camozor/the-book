#[allow(dead_code)]
pub fn execute_slice() {
    // let mut s1 = String::from("Hello");
    // let len = calculate_length(&s1);
    // println!("{len}");
    //
    // change(&mut s1);
    //
    // valid_stuff();

    // dangle();

    // let mut s = String::from("hello world");
    // {
    //     let first_word_index = better_first_word(&s);
    //     println!("{first_word_index}");
    // }
    // s.clear();

    // slices();
    
    // better_first_word("hello world");
    slice2();
}

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn change(s: &mut String) {
    s.push_str("toto");
}

// fn double_mut_unallowed() {
//     let mut s = String::from("Hello");
//
//     let r1 = &mut s;
//     let r2 = &mut s; // Multiple mutable references on the same data can not live simultaneously
//
//     println!("{}, {}", r1, r2);
// }

// fn mut_and_immut_unallow() {
//     let mut s = String::from("Hello");
//
//     let r1 = &s;
//     let r2 = &mut s; // Immutable and multiple references on the same data can not live
//                      // simultaneously
//
//     println!("{}, {}", r1, r2);
// }

#[allow(dead_code)]
fn valid_stuff() {
    let mut s = String::from("Hello");

    println!("{s}");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} & {r2} & {s}");
    println!("{r1} & {r2} & {s}");

    let r3 = &mut s;
    println!("{r3}");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//
//     &s // Can not return a reference to a dropped value
// }

#[allow(dead_code)]
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return index;
        }
    }

    s.len()
}

#[allow(dead_code)]
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}

#[allow(dead_code)]
fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}

#[allow(dead_code)]
fn slices() {
    let s = String::from("Hello world!");

    let s1 = &s[..5];
    let s2 = &s[6..11];

    println!("{s1}");
    println!("{s2}");
}

fn slice2() {
    let a = [1, 2, 3, 4];
    let slice = &a[1..2];

    assert_eq!(slice, [2]);
}

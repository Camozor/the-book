use core::panic;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn execute_chapter8() {
    execute_count_word();
}

#[allow(dead_code)]
fn execute_clean_get_access() {
    let v = vec![1, 2];
    let result = v.get(0);
    match result {
        Some(val) => println!("val = {}", val),
        None => println!("None"),
    }
}

#[allow(dead_code)]
fn execute_dirty_get_access() {
    let v = vec![1, 2];
    let result = v[3];
    println!("result = {}", result);
}

#[allow(dead_code)]
fn execute_invalid_reference_vector() {
    // let mut v = vec![1, 2, 3];
    //
    // let r = &v[0]; // Immutable borrow here
    // v.push(4); // Mutable borrow here
    // println!("{}", r); // Use after
}

#[allow(dead_code)]
fn execute_invalid_add_vector() {
    // let mut v = vec![1, 2, 3];
    //
    // for i in v {
    //     v.push(5); // Borrow of moved value v
    // }
}

#[allow(dead_code)]
fn execute_mutable_array_deref() {
    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        // *i *= 2; // Same
        *i = *i * 2;
    }
    println!("{:?}", v);
}

#[allow(dead_code)]
fn execute_string_stuff() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("{s2}");
}

#[allow(dead_code)]
fn execute_unicode_string() {
    let s = "ç".to_string();
    for b in s.bytes() {
        println!("{b}");
    }
    for c in s.chars() {
        println!("{c}");
    }
}

#[allow(dead_code)]
fn execute_hashmap() {
    let key = "bidule".to_string();
    let val = 9;

    let mut map = HashMap::new();
    map.insert(&key, val);

    println!("{}", key);
}

#[allow(dead_code)]
fn execute_count_word() {
    let text = "hello   bidule  world hello world world".to_string();

    let map = count_word(&text);
    println!("{:?}", map);
    panic!("gdsgsd");
}

fn count_word(text: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    map
}

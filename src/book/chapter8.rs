#[allow(dead_code)]
pub fn execute_chapter8() {
    execute_mutable_array_deref();
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

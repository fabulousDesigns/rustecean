fn main() {
    // creating a vector
    // 1. using Vec::new()
    let v: Vec<i32> = Vec::new();
    // 2. Using the vec! macro
    let v = vec![1, 2, 3];
    // modifying a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // accessing elements in a vector
    // 2 main ways
    // 1. using index syntax with []
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2]; // get the third element
    println!("The third element is {}", third);
    // 2. using the get method
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // The key difference between these two methods is how they handle invalid indices:
    // When using the [] method with an index that doesn't exist (like trying to access index 100 in a 5-element vector), the program will panic. This is best used when you want your program to crash if there's an attempt to access an element past the end of the vector. Rust-lang
    // When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector might happen occasionally under normal circumstances. Mit
    // iterating over vector Elements
    // immutable iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // mutable iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
    }
}
